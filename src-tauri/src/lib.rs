use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::task; 

// Import Base64 engine
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub cover: String, // Local path to image
    pub cover_color: String,
    pub status: String,
    #[serde(default)]
    pub total_pages: u32,
    #[serde(default)]
    pub pages_read: u32,
    pub date_added: String,
}

// A wrapper struct just for backups that includes the image data
#[derive(Serialize, Deserialize, Debug)]
struct BackupItem {
    #[serde(flatten)]
    book: Book,
    // Optional because the file might be missing or it's a web URL
    cover_base64: Option<String>, 
}

fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_data_dir = app_handle.path().app_data_dir().expect("failed to find app data dir");
    if !app_data_dir.exists() { fs::create_dir_all(&app_data_dir).expect("failed to create app data dir"); }
    app_data_dir.join("library.json")
}

fn get_covers_dir(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_data_dir = app_handle.path().app_data_dir().expect("failed to find app data dir");
    let covers_dir = app_data_dir.join("covers");
    if !covers_dir.exists() { fs::create_dir_all(&covers_dir).expect("failed to create covers dir"); }
    covers_dir
}

fn download_cover_image(app: &tauri::AppHandle, url: &str) -> Result<String, String> {
    if url.trim().is_empty() { return Err("empty url".into()); }

    let covers_dir = get_covers_dir(app);
    let ext = url.rsplit('.').next().filter(|e| e.len() <= 4).unwrap_or("jpg");
    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let dest_path = covers_dir.join(file_name);

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
    } else { vec![] }
}

#[tauri::command]
async fn add_book(
    app: tauri::AppHandle,
    title: String,
    author: String,
    cover: String,
    status: String,
    pages_read: u32,
    total_pages: u32,
) -> Result<Book, String> {
    let path = get_db_path(&app);
    let mut books = get_books(app.clone());

    if books.iter().any(|b| b.title.eq_ignore_ascii_case(&title) && b.author.eq_ignore_ascii_case(&author)) {
        return Err("Book already exists in library".to_string());
    }

    let app_clone = app.clone();
    let cover_url = cover.clone();
    let cover_path = task::spawn_blocking(move || {
        match download_cover_image(&app_clone, &cover_url) {
            Ok(local_path) => local_path,
            Err(_) => cover_url, 
        }
    }).await.map_err(|e| e.to_string())?;

    let new_book = Book {
        id: uuid::Uuid::new_v4().to_string(),
        title, author, cover: cover_path, cover_color: "#FF9A9E".to_string(),
        status, total_pages, pages_read, date_added: Utc::now().to_rfc3339(),
    };

    books.push(new_book.clone());
    let json = serde_json::to_string_pretty(&books).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;

    Ok(new_book)
}

#[tauri::command]
fn update_book_status(app: tauri::AppHandle, id: String, status: String) -> Result<(), String> {
    let path = get_db_path(&app);
    let mut books = get_books(app);
    if let Some(book) = books.iter_mut().find(|b| b.id == id) {
        book.status = status;
        let json = serde_json::to_string_pretty(&books).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    } else { Err("Book not found".to_string()) }
}

#[tauri::command]
fn delete_book(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_db_path(&app);
    let mut books = get_books(app);
    if let Some(idx) = books.iter().position(|b| b.id == id) {
        let book = &books[idx];
        if !book.cover.starts_with("http") {
            let _ = fs::remove_file(&book.cover);
        }
        books.remove(idx);
        let json = serde_json::to_string_pretty(&books).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    } else { Err("Book not found".to_string()) }
}

#[tauri::command]
fn update_book_progress(app: tauri::AppHandle, id: String, pages_read: u32) -> Result<(), String> {
    let path = get_db_path(&app);
    let mut books = get_books(app);
    if let Some(book) = books.iter_mut().find(|b| b.id == id) {
        book.pages_read = pages_read;
        if book.total_pages > 0 && book.pages_read >= book.total_pages {
            book.status = "finished".to_string();
            book.pages_read = book.total_pages; 
        } else if book.pages_read > 0 && book.status == "to-read" {
            book.status = "reading".to_string();
        }
        let json = serde_json::to_string_pretty(&books).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    } else { Err("Book ID not found".to_string()) }
}

// --- INTELLIGENT BACKUP & RESTORE ---

#[tauri::command]
async fn backup_library(app: tauri::AppHandle, target_path: String) -> Result<(), String> {
    let books = get_books(app);
    
    // Create backup items with embedded images
    let backup_items: Vec<BackupItem> = books.into_iter().map(|b| {
        let mut base64_data = None;
        // Try to read the file if it's a local path
        if !b.cover.starts_with("http") && !b.cover.is_empty() {
            if let Ok(bytes) = fs::read(&b.cover) {
                base64_data = Some(BASE64.encode(bytes));
            }
        }
        BackupItem { book: b, cover_base64: base64_data }
    }).collect();

    let json = serde_json::to_string_pretty(&backup_items).map_err(|e| e.to_string())?;
    fs::write(target_path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn restore_library(app: tauri::AppHandle, source_path: String) -> Result<(), String> {
    let db_path = get_db_path(&app);
    let covers_dir = get_covers_dir(&app);

    let content = fs::read_to_string(&source_path).map_err(|e| e.to_string())?;
    
    // Try to parse as the new BackupItem format (with images)
    // If that fails, fallback to simple Vec<Book> (legacy backup)
    let books: Vec<Book> = match serde_json::from_str::<Vec<BackupItem>>(&content) {
        Ok(items) => {
            // Restore images to disk
            items.into_iter().map(|item| {
                let mut book = item.book;
                
                // If we have image data, save it to the new covers directory
                if let Some(b64) = item.cover_base64 {
                    if let Ok(bytes) = BASE64.decode(b64) {
                        let ext = "jpg"; // Default extension
                        let new_filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);
                        let new_path = covers_dir.join(new_filename);
                        
                        // Write image to disk
                        if fs::write(&new_path, bytes).is_ok() {
                            // Update book to point to the new valid path
                            book.cover = new_path.to_string_lossy().to_string();
                        }
                    }
                }
                book
            }).collect()
        },
        Err(_) => {
            // Fallback: Use standard parse if backup has no images (old version)
             serde_json::from_str::<Vec<Book>>(&content).map_err(|_| "Invalid backup file".to_string())?
        }
    };

    // Save the restored library (with corrected paths)
    let final_json = serde_json::to_string_pretty(&books).map_err(|e| e.to_string())?;
    fs::write(db_path, final_json).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn delete_library(app: tauri::AppHandle) -> Result<(), String> {
    let db_path = get_db_path(&app);
    fs::write(db_path, "[]").map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn update_book_cover(app: tauri::AppHandle, id: String, cover: String) -> Result<(), String> {
    // 1. Get the path to library.json manually (same way get_books does it)
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let library_path = app_data_dir.join("library.json");

    // 2. Read existing books
    let mut books: Vec<Book> = if library_path.exists() {
        let file = std::fs::File::open(&library_path).map_err(|e| e.to_string())?;
        serde_json::from_reader(file).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    // 3. Find and update the book
    if let Some(book) = books.iter_mut().find(|b| b.id == id) {
        book.cover = cover;
        
        // 4. Save back to JSON
        let json = serde_json::to_string_pretty(&books).map_err(|e| e.to_string())?;
        std::fs::write(library_path, json).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Book not found".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init()) 
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_books, add_book, update_book_status, delete_book, 
            update_book_progress, backup_library, restore_library, delete_library, update_book_cover
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
