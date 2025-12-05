use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    /// Local path to the saved cover image (or original URL if download failed)
    pub cover: String,
    pub cover_color: String,
    pub status: String,
    #[serde(default)]
    pub pages_read: u32,
    pub date_added: String,
}

fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to find app data dir");
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
    }
    app_data_dir.join("library.json")
}

fn get_covers_dir(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to find app data dir");
    let covers_dir = app_data_dir.join("covers");
    if !covers_dir.exists() {
        fs::create_dir_all(&covers_dir).expect("failed to create covers dir");
    }
    covers_dir
}

/// Download the cover image from `url` into the covers directory and return the local path.
/// If anything fails, return an Err so the caller can fall back to the URL.[web:816][web:813]
fn download_cover_image(app: &tauri::AppHandle, url: &str) -> Result<String, String> {
    if url.trim().is_empty() {
        return Err("empty url".into());
    }

    let covers_dir = get_covers_dir(app);

    // Infer a simple extension from the URL (default to jpg)
    let ext = url
        .rsplit('.')
        .next()
        .filter(|e| e.len() <= 4)
        .unwrap_or("jpg");

    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let dest_path = covers_dir.join(file_name);

    // Blocking download is fine in a tauri command.[web:816][web:813]
    let mut response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    let mut file = File::create(&dest_path).map_err(|e| e.to_string())?;
    copy(&mut response, &mut file).map_err(|e| e.to_string())?;

    Ok(dest_path.to_string_lossy().to_string())
}

#[tauri::command]
fn get_books(app: tauri::AppHandle) -> Vec<Book> {
    let path = get_db_path(&app);
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

#[tauri::command]
fn add_book(
    app: tauri::AppHandle,
    title: String,
    author: String,
    cover: String,      // URL from frontend
    status: String,
    pages_read: u32,
) -> Book {
    let path = get_db_path(&app);
    let mut books = get_books(app.clone());

    // Try to download the cover; if it fails, keep the original URL.
    let cover_path = match download_cover_image(&app, &cover) {
        Ok(local) => local,
        Err(_) => cover,
    };

    let new_book = Book {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        author,
        cover: cover_path,
        cover_color: "#FF9A9E".to_string(),
        status,
        pages_read,
        date_added: Utc::now().to_rfc3339(),
    };

    books.push(new_book.clone());

    let json = serde_json::to_string_pretty(&books).expect("failed to save");
    fs::write(path, json).expect("failed to write file");
    new_book
}

#[tauri::command]
fn update_book_status(app: tauri::AppHandle, id: String, status: String) -> Result<(), String> {
    let path = get_db_path(&app);

    if !path.exists() {
        return Err("Library file not found".to_string());
    }

    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut books: Vec<Book> = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    if let Some(book) = books.iter_mut().find(|b| b.id == id) {
        book.status = status;
    } else {
        return Err("Book not found".to_string());
    }

    let new_data = serde_json::to_string_pretty(&books).map_err(|e| e.to_string())?;
    fs::write(path, new_data).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn delete_book(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_db_path(&app);

    if !path.exists() {
        return Err("Library file not found".to_string());
    }

    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut books: Vec<Book> = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    let initial_len = books.len();
    books.retain(|b| b.id != id);

    if books.len() == initial_len {
        return Err("Book not found".to_string());
    }

    let new_data = serde_json::to_string_pretty(&books).map_err(|e| e.to_string())?;
    fs::write(path, new_data).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_books,
            add_book,
            update_book_status,
            delete_book
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
