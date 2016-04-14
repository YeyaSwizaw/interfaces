use ::{Point, Rect, UI, WidgetIndex};

pub trait Widget<RenderArgs> {
   fn calculate_bounds(&self, ui: &UI<RenderArgs>) -> Rect;
   fn dependencies(&self) -> &[WidgetIndex];

   fn as_renderable(&self) -> Option<&Renderable<RenderArgs>> { 
       None 
   }

   fn as_clickable(&self) -> Option<&Clickable> {
       None 
   }

   fn as_clickable_mut(&mut self) -> Option<&mut Clickable> {
       None
   }
}

pub trait Clickable {
    fn on_click(&mut self, point: Point);
}

pub trait Renderable<RenderArgs> {
    fn render(&self, bounds: &Rect, args: RenderArgs);
}
