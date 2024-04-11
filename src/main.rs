use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc, Data, Lens, Env, Command, Target};

// Define your application state
#[derive(Data, Clone, Lens)]
struct AppState {
    input_text: String,
}

fn build_ui() -> impl Widget<AppState> {
    // Create a vertical Flex container
    let layout = Flex::column()
        .with_child(
            // Add a label
            Label::new("Hello, Rust GUI!").center(),
        )
        .with_spacer(8.0)
        .with_child(
            // Add a TextBox for text input
            TextBox::new()
                .with_placeholder("Type something...")
                .lens(AppState::input_text)
                .fix_width(200.0),
        )
        .with_spacer(8.0)
        .with_child(
            // Add a button to save text
            Button::new("Save")
                .on_click(|_ctx, data: &mut AppState, _env| {
                    // Functionality to save text to a file
                    std::fs::write("output.txt", &data.input_text).expect("Unable to write file");
                })
                .fix_width(100.0),
        );

    layout
}

fn main() {
    // Describe the main window
    let main_window = WindowDesc::new(build_ui)
        .title("Simple Rust GUI with Text Save Functionality")
        .window_size((400.0, 300.0));

    // Launch the application with initial state
    AppLauncher::with_window(main_window)
        .launch(AppState {
            input_text: String::new(),
        })
        .expect("Failed to launch application");
}
