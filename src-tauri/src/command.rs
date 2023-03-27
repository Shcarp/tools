use serde_json::Value;
use tauri::{AppHandle, Manager, Window, Error};

use crate::rpc::{NServer, NRequest, NResponse};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn recive_message(server: tauri::State<'_, NServer>,  request: NRequest) -> Result<NResponse, ()> {
    println!("{:?}", request);
    Ok(server.call(request).await?)
}

#[tauri::command]
pub async fn open(app: AppHandle, win: Window, name: String, args: Value) -> Result<(), Error> {
    let url = format!("{}.html", name);
    if let Some(nwin) = app.get_window(&name) {
        win.hide()?;
        nwin.show()
    } else {
        let window = tauri::WindowBuilder::new(
            &app,
            name, /* the unique window label */
            tauri::WindowUrl::App(url.into())
        ).initialization_script(
            format!(
                r#"
                    if (window.location.origin === 'http://localhost:1420') {{
                        console.log("hello world from js init script");

                        window.__MY_CUSTOM_PROPERTY__ = '{}';
                    }}
                "#, args).as_str()
        )
        .build()?;
        win.hide()?;
        window.show()?;
        Ok(())
    }
}
