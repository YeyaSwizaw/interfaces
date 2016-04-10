#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: u64,
    pub y: u64
}

fn point(x: u64, y: u64) -> Point {
    Point {
        x: x,
        y: y
    }
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub top_left: Point,
    pub bot_right: Point
}

trait Widget {
   fn calculate_bounds(&self, ui: &UI) -> Bounds;
   fn dependencies(&self) -> &[WidgetIndex];
   fn render(&self, bounds: &Bounds);
}

struct UI {
    widgets: Vec<Box<Widget>>,
    widget_bounds: Vec<Bounds>,
    idxs: Vec<usize>,

    bounds: Bounds
}

#[derive(Debug, Copy, Clone)]
struct WidgetIndex(usize);

impl UI {
    fn new(bounds: Bounds) -> UI {
        UI {
            widgets: Vec::new(),
            widget_bounds: Vec::new(),
            idxs: Vec::new(),
            
            bounds: bounds
        }
    }

    fn add_widget(&mut self, new: Box<Widget>) -> WidgetIndex {
        let mut idx = 0;

        let deps: Vec<usize> = new.dependencies().iter().map(|idx| self.idxs[idx.0]).collect();
        let mut found = Vec::new();
        
        // Find insert location
        for (i, _) in self.widgets.iter().enumerate() {
            if deps.contains(&i) {
                found.push(i);
            }

            if found.len() == deps.len() {
                idx = i + 1;
                break
            }
        }

        self.widgets.insert(idx, new);

        for i in self.idxs.iter_mut().filter(|i| **i >= idx) {
            *i += 1
        }

        self.idxs.push(idx);
        WidgetIndex(self.idxs.len() - 1)
    }

    fn get_widget_and_bounds(&self, idx: WidgetIndex) -> (&Box<Widget>, &Bounds) {
        let idx = self.idxs[idx.0];
        (&self.widgets[idx], &self.widget_bounds[idx])
    }

    fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn resize(&mut self, bounds: Bounds) {
        self.bounds = bounds;
        self.recalculate()
    }

    fn recalculate(&mut self) {
        self.widget_bounds.clear();

        for widget in self.widgets.iter() {
            let bounds = widget.calculate_bounds(self);
            self.widget_bounds.push(bounds)
        }
    }

    fn render(&self) {
        for (widget, bounds) in self.widgets.iter().zip(self.widget_bounds.iter()) {
            widget.render(bounds)
        }
    }
}

#[test]
fn test() {
    struct Test {
        deps: Vec<WidgetIndex>,
        bounds: Box<Fn(&Test, &UI) -> Bounds>
    }

    impl Widget for Test {
        fn calculate_bounds(&self, ui: &UI) -> Bounds {
            (self.bounds)(self, ui)
        }

        fn dependencies(&self) -> &[WidgetIndex] {
            &self.deps
        }

        fn render(&self, bounds: &Bounds) {
            println!("{:?}", bounds);
        }
    }

    impl Test {
        fn new<F>(deps: &[WidgetIndex], f: F) -> Test 
            where F: 'static + Fn(&Test, &UI) -> Bounds {

            Test {
                deps: deps.to_owned(),
                bounds: Box::new(f),
            }
        }
    }

    let mut ui = UI::new(Bounds {
        top_left: point(0, 0),
        bot_right: point(800, 600)
    });

    println!("{:?}", ui.idxs);
    println!("");

    let t1 = ui.add_widget(Box::new(Test::new(&[], |_, ui| {
        let bounds = ui.bounds();
        Bounds {
            top_left: point(bounds.top_left.x + 16, bounds.top_left.y + 16),
            bot_right: point(bounds.top_left.x + 48, bounds.top_left.y + 48)
        }
    })));

    println!("{:?}", t1);
    println!("{:?}", ui.idxs);
    println!("");

    let t2 = ui.add_widget(Box::new(Test::new(&[], |_, ui| {
        let bounds = ui.bounds();
        Bounds {
            top_left: point(bounds.bot_right.x - 48, bounds.bot_right.y - 48),
            bot_right: point(bounds.bot_right.x - 16, bounds.bot_right.y - 16)
        }
    })));

    println!("{:?}", t2);
    println!("{:?}", ui.idxs);
    println!("");

    let t3 = ui.add_widget(Box::new(Test::new(&[t1], move |_, ui| {
        let (_, bounds) = ui.get_widget_and_bounds(t1);
        Bounds {
            top_left: point(bounds.top_left.x + 16, bounds.top_left.y + 16),
            bot_right: point(bounds.top_left.x + 48, bounds.top_left.y + 48)
        }
    })));

    println!("{:?}", t3);
    println!("{:?}", ui.idxs);
    println!("");

    ui.recalculate();
    ui.render();
}
