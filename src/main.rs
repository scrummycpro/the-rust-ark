use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::{AppLauncher, Data, Env, EventCtx, Lens, Widget, WidgetExt, WindowDesc};
use rusqlite::{Connection, params};
use std::sync::Arc;
use chrono::{Local, Datelike, Timelike};


#[derive(Clone, Data, Lens, Debug)]
struct AppState {
    input_text: String,
    info_text: String,
    entries: Arc<Vec<Entry>>,
}

#[derive(Clone, Data, Debug)]
struct Entry {
    id: i32,
    quark: String,
    timestamp: String,
}

fn setup_database() -> Connection {
    let conn = Connection::open("app_data.db").expect("failed to open db");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (id INTEGER PRIMARY KEY, quark TEXT, timestamp TEXT)",
        params![],
    ).expect("failed to create table");
    conn
}

fn fetch_entries(conn: &Connection) -> Vec<Entry> {
    let mut stmt = conn.prepare("SELECT id, quark, timestamp FROM entries").expect("prepare failed");
    let entries_iter = stmt.query_map(params![], |row| {
        Ok(Entry {
            id: row.get(0)?,
            quark: row.get(1)?,
            timestamp: row.get(2)?,
        })
    }).expect("query map failed");

    entries_iter.map(|entry| entry.expect("entry fetch failed")).collect()
}

fn insert_entry(conn: &Connection, quark: &str, timestamp: &str) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO entries (quark, timestamp) VALUES (?1, ?2)",
        params![quark, timestamp],
    )
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
                .on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env| {
                    println!("Save button clicked");
                    // Implement actual save logic here
                    let conn = setup_database();
                    conn.execute(
                        "INSERT INTO entries (quark, timestamp) VALUES (?1, ?2)",
                        params![data.input_text, Local::now().to_string()],
                    )
                    .expect("failed to save note to database");
                    data.info_text = "Quark saved".to_string();
                    data.input_text.clear();
                })
                .fix_width(100.0),
        )
        .with_spacer(8.0)
        .with_child(
            Button::new("Show Entries")
                .on_click(|ctx: &mut EventCtx, data: &mut AppState, _env| {
                    println!("Show Entries button clicked");
                    let conn = setup_database();
                    let entries = fetch_entries(&conn);
                    data.entries = Arc::new(entries);
                    println!("Entries fetched: {:?}", data.entries);
                    ctx.new_window(build_entry_window());
                    ctx.request_update();  // Ensure UI updates with new data
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



fn build_entry_window() -> WindowDesc<AppState> {
    WindowDesc::new((|| {
        // Inside this closure, create the widget that you want to return
        Scroll::new(
            List::new(|| {
                Flex::row()
                    .with_child(Label::dynamic(|entry: &Entry, _: &Env| format!("{} - {}", entry.timestamp, entry.quark)))
                    .padding(10.0)
                    .expand_width()
            })
            .lens(AppState::entries)
        )
        .vertical()
        .padding(10.0)
        // The closure should directly return this widget, without any trailing code
    })())  // The closure is defined and immediately called
    .title("Database Entries")
    .window_size((400.0, 300.0))
}
fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Rust GUI with SQLite Integration")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(AppState {
            input_text: String::new(),
            info_text: String::new(),
            entries: Arc::new(Vec::new()), // Initially empty
        })
        .expect("Failed to launch application");
}
