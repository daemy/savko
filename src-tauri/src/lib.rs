use std::fs::{remove_file, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::thread::{sleep, spawn};
use std::time::Duration;
use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;

#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    message: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct ClipboardHistory {
    items: Vec<Notification>,
}

const PATH : &str = "data/clipboard_history.json";

#[tauri::command]
fn wipe_all() {
    let _ = remove_file(PATH);
}

#[tauri::command]
fn copy(data: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(data).unwrap()
}

#[tauri::command]
fn load_last_n_entries(n: usize) -> Vec<Notification> {
    if let Ok(history) = load_history() {
        history.items.into_iter().rev().take(n).collect()
    } else {
        vec![]
    }
}

#[tauri::command]
fn init(on_event : Channel<String>) {
    spawn(move || {
        let mut clipboard = Clipboard::new().unwrap();
        let mut last_clipboard: Option<String> = None;

        loop {
            if let Ok(data) = clipboard.get_text() {
                if last_clipboard.as_ref() != Some(&data) {
                    last_clipboard = Some(data.clone());
                    println!("{}", data.clone());
                    let mut history = load_history().unwrap_or_else(|_| ClipboardHistory { items: vec![] });
                    history.items.push(Notification { message: data.clone(), description: "Copied to clipboard".to_string() });
                    save_history(&history).unwrap();
                    on_event.send(data).unwrap()
                }
            }
            sleep(Duration::from_secs(1))
        }
    });
}

fn load_history() -> Result<ClipboardHistory, std::io::Error> {
    let file = File::open(PATH)?;
    let reader = BufReader::new(file);
    let history = serde_json::from_reader(reader)?;
    Ok(history)
}

fn save_history(history: &ClipboardHistory) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(PATH)?;

    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, history)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![wipe_all, copy, load_last_n_entries, init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}