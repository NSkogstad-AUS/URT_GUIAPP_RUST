use druid::widget::{Label, Flex};
use druid::{AppLauncher, Widget, WindowDesc, Data, Lens};

#[derive(Clone, Data, Lens)]
struct AppState {
    label_text: String,
}

fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new(|data: &AppState, _env: &_| format!("{}", data.label_text)))
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Hello Druid");

    let initial_state = AppState {
        label_text: "Hello, world!".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}