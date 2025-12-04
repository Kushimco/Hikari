use std::fs;
use std::path::PathBuf;
use tauri::Manager;
// Explicitly import the traits needed for JSON conversion
use serde::{Serialize, Deserialize}; 


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub cover: String,
    pub cover_color: String,
    pub status: String,
}

fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_data_dir = app_handle.path().app_data_dir().expect("failed to find app data dir");
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
    }
    app_data_dir.join("library.json")
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
fn add_book(app: tauri::AppHandle, title: String, author: String, cover: String) -> Book {
    let path = get_db_path(&app);
    let mut books = get_books(app.clone());
    
    let new_book = Book {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        author,
        cover,
        cover_color: "#FF9A9E".to_string(), 
        status: "to-read".to_string(),
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
    
    //Find and Update Book
    if let Some(book) = books.iter_mut().find(|b| b.id == id) {
        book.status = status;
    } else {
        return Err("Book not found".to_string());
    }
    
    //Write back to file
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
        .invoke_handler(tauri::generate_handler![get_books, add_book, update_book_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
