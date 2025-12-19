use gpui::*;

struct FileTimeUpdater;

impl Render for FileTimeUpdater {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x1e1e2e))
            .size_full()
            .justify_center()
            .items_center()
            .text_color(rgb(0xeeeeee))
            .child(
                div()
                    .w(px(500.))
                    .h(px(400.))
                    .bg(rgb(0x313244))
                    .rounded_lg()
                    .shadow_lg()
                    .p_4()
                    .child("Let's see if this works...")
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let mut options = WindowOptions::default();
        options.title = Some("File Time Updater".into());
        options.bounds = Some(WindowBounds::Fixed(Bounds {
            origin: Default::default(),
            size: size(px(600.), px(500.)).into(),
        }));

        cx.open_window(options, |cx| {
            cx.new_view(|_cx| FileTimeUpdater)
        });
    });
}