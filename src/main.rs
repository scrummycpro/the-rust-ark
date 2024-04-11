use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc, Data, Lens, Env};
use rusqlite::{params, Connection, Result};
use chrono::Local;

#[derive(Data, Clone, Lens)]
struct AppState {
    input_text: String,
    info_text: String,
}

fn build_ui() -> impl Widget<AppState> {
    let layout = Flex::column()
        .with_child(Label::new("Hello, Rust GUI!").center())
        .with_spacer(8.0)
        .with_child(
            TextBox::new()
                .with_placeholder("Enter quark here")
                .lens(AppState::input_text)
                .fix_width(300.0)
                .fix_height(100.0),
        )
        .with_spacer(8.0)
        .with_child(
            Button::new("Save")
                .on_click(|ctx, data: &mut AppState, _env| {
                    if let Err(e) = add_record(&data.input_text) {
                        data.info_text = format!("Error: {}", e);
                    } else {
                        data.info_text = "Quark saved".to_string();
                        data.input_text.clear();
                    }
                    ctx.request_update();
                })
                .fix_width(100.0),
        )
        .with_spacer(8.0)
        .with_child(
            Label::new(|data: &AppState, _env: &Env| data.info_text.clone())
                .center(),
        );

    layout
}

fn main() {
    setup_database().expect("Failed to set up database");

    let main_window = WindowDesc::new(build_ui())
        .title("Rust GUI with SQLite Integration")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(AppState {
            input_text: String::new(),
            info_text: String::new(),
        })
        .expect("Failed to launch application");
}

fn add_record(text: &str) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("app_data.db")?;
    let timestamp = Local::now().to_rfc3339();
    conn.execute(
        "INSERT INTO entries (quark, timestamp) VALUES (?1, ?2)",
        params![text, timestamp],
    )?;
    Ok(())
}

fn setup_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("app_data.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (id INTEGER PRIMARY KEY, quark TEXT, timestamp TEXT)",
        [],
    )?;
    Ok(())
}
