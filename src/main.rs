use druid::widget::{Label, Flex};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};

fn build_ui() -> impl Widget<()> {
    // Create a vertical Flex container and add a label to it
    Flex::column()
        .with_child(Label::new("Hello, Rust GUI!").center())
}

fn main() {
    // Describe the main window
    let main_window = WindowDesc::new(|| build_ui())
        .title("Simple Rust GUI")
        .window_size((400.0, 200.0));

    // Launch the application
    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");
}
