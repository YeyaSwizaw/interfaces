use ::{Rect, UI, WidgetIndex};

pub trait Widget {
   fn calculate_bounds(&self, ui: &UI) -> Rect;
   fn dependencies(&self) -> &[WidgetIndex];
   fn render(&self, bounds: &Rect);
}

