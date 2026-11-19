use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::label::Label;
use gpui_kit::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .size_full()
            .items_center()
            .justify_center()
            .child(Label::new("Hello, World!"))
            .child(Button::new("go").primary().label("Let's Go!"))
    }
}

fn main() {
    application().run(|cx| {
        init(cx);
        let bounds = Bounds::centered(None, size(px(500.0), px(300.0)), cx);
        open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Bamboo Launcher".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            cx,
            |_, cx| cx.new(|_| HelloWorld),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
