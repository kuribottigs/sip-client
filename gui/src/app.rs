use crate::views::MainView;
use gpui::*;

pub fn run() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(320.), px(420.0)), cx);

        let window_options = WindowOptions {
            app_id: Some("com.samesama.rphone".to_string()),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Phone App")),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        cx.open_window(window_options, |_, cx| MainView::new(cx))
            .unwrap();
    });
}
