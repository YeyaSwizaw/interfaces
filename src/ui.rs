use ::widget::traits::{Widget, Renderable, Clickable};
use ::{Point, Rect};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WidgetIndex(usize);

pub struct UI {
    widgets: Vec<Box<Widget>>,
    widget_bounds: Vec<Rect>,

    idxs: Vec<usize>,
    renderable_idxs: Vec<usize>,
    clickable_idxs: Vec<usize>,

    bounds: Rect
}

impl UI {
    pub fn new(bounds: Rect) -> UI {
        UI {
            widgets: Vec::new(),
            widget_bounds: Vec::new(),

            idxs: Vec::new(),
            renderable_idxs: Vec::new(),
            clickable_idxs: Vec::new(),
            
            bounds: bounds
        }
    }

    pub fn add_widget(&mut self, new: Box<Widget>) -> WidgetIndex {
        let widget_index = self.idxs.len();

        // Add components
        if let Some(_) = new.as_renderable() {
            self.renderable_idxs.push(widget_index)
        }

        if let Some(_) = new.as_clickable() {
            self.clickable_idxs.push(widget_index)
        }

        let deps: Vec<usize> = new.dependencies().iter().map(|idx| self.idxs[idx.0]).collect();
        let mut found = Vec::new();
        
        // Find insert location
        let mut idx = 0;
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
        WidgetIndex(widget_index)
    }

    pub fn get_widget_and_bounds(&self, idx: WidgetIndex) -> (&Box<Widget>, &Rect) {
        let idx = self.idxs[idx.0];
        (&self.widgets[idx], &self.widget_bounds[idx])
    }

    pub fn get_widget(&self, idx: WidgetIndex) -> &Box<Widget> {
        let idx = self.idxs[idx.0];
        &self.widgets[idx]
    }

    pub fn get_widget_mut(&mut self, idx: WidgetIndex) -> &mut Box<Widget> {
        let idx = self.idxs[idx.0];
        &mut self.widgets[idx]
    }

    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }

    pub fn resize(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.recalculate()
    }

    pub fn recalculate(&mut self) {
        self.widget_bounds.clear();

        for widget in self.widgets.iter() {
            let bounds = widget.calculate_bounds(self);
            self.widget_bounds.push(bounds)
        }
    }

    pub fn render(&self) {
        for renderable_idx in self.renderable_idxs.iter() {
            let idx = self.idxs[*renderable_idx];
            let bounds = self.widget_bounds[idx].clone();
            let widget = &self.widgets[idx];
            widget.as_renderable().unwrap().render(&bounds);
        }
    }

    pub fn click(&mut self, point: Point) {
        for clickable_idx in self.clickable_idxs.iter() {
            let idx = self.idxs[*clickable_idx];
            let bounds = self.widget_bounds[idx].clone();
            if bounds.contains(point) {
                let mut widget = &mut self.widgets[idx];
                widget.as_clickable_mut().unwrap().on_click(point);
            }
        }
    }
}

#[test]
fn test() {
    use ::Point;

    struct Test {
        deps: Vec<WidgetIndex>,
        bounds: Box<Fn(&Test, &UI) -> Rect>
    }

    impl Widget for Test {
        fn calculate_bounds(&self, ui: &UI) -> Rect {
            (self.bounds)(self, ui)
        }

        fn dependencies(&self) -> &[WidgetIndex] {
            &self.deps
        }

        fn as_renderable(&self) -> Option<&Renderable> {
            Some(self as &Renderable)
        }
    }

    impl Renderable for Test {
        fn render(&self, bounds: &Rect) {
            println!("{:?}", bounds);
        }
    }

    impl Test {
        fn new<F>(deps: &[WidgetIndex], f: F) -> Test 
            where F: 'static + Fn(&Test, &UI) -> Rect {

            Test {
                deps: deps.to_owned(),
                bounds: Box::new(f),
            }
        }
    }

    let mut ui = UI::new(Rect {
        top_left: Point(0, 0),
        bot_right: Point(800, 600)
    });

    println!("{:?}", ui.idxs);
    println!("");

    let t1 = ui.add_widget(Box::new(Test::new(&[], |_, ui| {
        let bounds = ui.bounds();
        Rect {
            top_left: Point(bounds.top_left.x + 16, bounds.top_left.y + 16),
            bot_right: Point(bounds.top_left.x + 48, bounds.top_left.y + 48)
        }
    })));

    println!("{:?}", t1);
    println!("{:?}", ui.idxs);
    println!("");

    let t2 = ui.add_widget(Box::new(Test::new(&[], |_, ui| {
        let bounds = ui.bounds();
        Rect {
            top_left: Point(bounds.bot_right.x - 48, bounds.bot_right.y - 48),
            bot_right: Point(bounds.bot_right.x - 16, bounds.bot_right.y - 16)
        }
    })));

    println!("{:?}", t2);
    println!("{:?}", ui.idxs);
    println!("");

    let t3 = ui.add_widget(Box::new(Test::new(&[t1], move |_, ui| {
        let (_, bounds) = ui.get_widget_and_bounds(t1);
        Rect {
            top_left: Point(bounds.top_left.x + 16, bounds.top_left.y + 16),
            bot_right: Point(bounds.top_left.x + 48, bounds.top_left.y + 48)
        }
    })));

    println!("{:?}", t3);
    println!("{:?}", ui.idxs);
    println!("");

    ui.recalculate();
    ui.render();
}
