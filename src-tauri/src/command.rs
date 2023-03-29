use tauri::AppHandle;

use crate::rpc::{NServer, NRequest, NResponse};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn recive_message(app: AppHandle, server: tauri::State<'_, NServer>,  request: NRequest) -> Result<NResponse, ()> {
    println!("{:?}", request);
    Ok(server.call(request).await?)
}
