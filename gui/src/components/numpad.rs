use gpui::{prelude::*, Context, SharedString, Window, div, px, rgb};

pub struct NumPad {
    pub display: String,
}

impl NumPad {
    pub fn new() -> Self {
        Self {
            display: String::new(),
        }
    }
}

fn key_button<F>(label: &'static str, cx: &mut Context<NumPad>, on_click: F) -> impl IntoElement + use<F>
where
    F: Fn(&mut NumPad, &mut Context<NumPad>) + 'static,
{
    div()
        .id(SharedString::from(label))
        .flex()
        .items_center()
        .justify_center()
        .size(px(70.0))
        .rounded_md()
        .bg(rgb(0x3a3a3a))
        .text_color(rgb(0xffffff))
        .text_xl()
        .cursor_pointer()
        .hover(|this| this.bg(rgb(0x555555)))
        .active(|this| this.bg(rgb(0x222222)))
        .on_click(cx.listener(move |this, _event, _window, cx| on_click(this, cx)))
        .child(label)
}

impl Render for NumPad {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let keys = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "*", "0", "#",
        ];

        div()
            .flex()
            .flex_col()
            .gap_3()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x202020))
            .p_4()
            .child(
                div()
                    .w(px(244.0))
                    .h(px(50.0))
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_3()
                    .rounded_md()
                    .bg(rgb(0x000000))
                    .text_color(rgb(0xffffff))
                    .text_2xl()
                    .child(self.display.clone()),
            )
            .child(
                div().grid().grid_cols(3).gap_3().children(
                    keys.into_iter()
                        .map(|key| key_button(key, &mut *cx, move |this, cx| {
                            this.display.push_str(key);
                            cx.notify();
                        })),
                ),
            )
            .child(
                div()
                    .id("clear")
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(244.0))
                    .h(px(40.0))
                    .rounded_md()
                    .bg(rgb(0x6a2a2a))
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .hover(|this| this.bg(rgb(0x8a3a3a)))
                    .active(|this| this.bg(rgb(0x4a1a1a)))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.display.clear();
                        cx.notify();
                    }))
                    .child("Clear"),
            )
    }
}
