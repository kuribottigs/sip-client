use gpui::{prelude::*, App, Context, Entity, Window, div};
use crate::components::NumPad;

pub struct MainView {
    numpad: Entity<NumPad>,
}

impl MainView {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let numpad = cx.new(|_| NumPad::new());
        cx.new(|_| MainView { numpad })
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.numpad.clone())
    }
}
