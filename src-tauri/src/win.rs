use std::{collections::HashMap, fmt::Display, sync::Mutex, f32::consts::E};
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Manager, Window, Error, State};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum WinError {
  #[error("Options `{0}` is not find")]
  MissOptions(String),
  #[error("{0} Already exist")]
  AlreadyExist(String),
  #[error("{0} Already open")]
  AlreadyOpen(String),
  #[error("{0} Already hide")]
  AlreadyHide(String),
}

#[derive(Serialize, Deserialize)]
struct NArgs {
    send: String,
    args: HashMap<String, Value>
}

impl NArgs {
    pub fn new(send: &str, args: HashMap<String, Value>) -> String {
        NArgs {send: send.to_string(), args}.to_string()
    }
}

impl Display for NArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json_str = serde_json::to_string(self).unwrap();
        write!(f, "{}", json_str)
    }
}

pub struct WinOptions {
    pub win_type: String,
    pub overopen: bool,
    pub url: String,
    pub position: (i32, i32),
    pub height: i32,
    pub width: i32,
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: i32,
    pub max_height: i32,
    pub resizable: bool,
    pub title: String,
    pub fullscreen: bool,
}

pub struct WinState {
    pub register_win_types: Mutex<HashMap<String, WinOptions>>,
    pub open_wins: Mutex<Vec<String>>,
    pub hide_wins: Mutex<Vec<String>>
}

unsafe impl Send for WinState {}
unsafe impl Sync for WinState {}

impl WinState {
    pub fn new() -> Self {
        Self {
            register_win_types: Mutex::new(HashMap::new()),
            open_wins: Mutex::new(vec![]) ,
            hide_wins: Mutex::new(vec![]) 
        }
    }

    fn is_open(&self, label: &str) -> Result<bool, ()> {
        if let Ok(ref mut wins) = self.open_wins.lock() {
            Ok(wins.iter().find(|&win| {*win == label.to_string()}).is_some()) 
        } else { 
            Err(())
        }
        // self.open_wins.lock().iter().find(|&win| {*win == label.to_string()}).is_some()
    }
    // fn is_hide(&self, label: &str) -> bool {
    //     self.hide_wins.lock().unwrap().iter().find(|&win| {*win == label.to_string()}).is_some()
    // }
    fn is_register(&self, win_type: &str) -> bool {
        self.register_win_types.lock().unwrap().get(win_type).is_some()
    }

    // pub fn get_options_by_type(&'static self, win_type: &str) -> Option<&WinOptions> {
    //     self.register_win_types.lock().unwrap().get(win_type)
    // }

    pub fn register(&mut self, options: WinOptions)-> Result<(), WinError> {
        if self.is_register(&options.win_type) {
            return Err(WinError::AlreadyExist(format!("Win type {}", &options.win_type)))
        }
        self.register_win_types.lock().unwrap().insert(options.win_type.clone(), options);
        Ok(())
    }

    // pub fn open(&mut self, label: &str)-> Result<(), WinError> {
    //     if self.is_open(label) {
    //         return Err(WinError::AlreadyOpen(label.to_string()));
    //     }
    //     if self.is_hide(label) {
    //         self.hide_wins.lock().unwrap().retain(|x| {*x != label})
    //     }
    //     self.open_wins.lock().unwrap().push(label.to_string());
    //     Ok(())
    // }

    // pub fn hide(&mut self, label: &str)-> Result<(), WinError> {
        // if self.is_hide(label) {
        //     return Err(WinError::AlreadyHide(label.to_string()));
        // }
        // if self.is_open(label) {
        //     self.open_wins.lock().unwrap().retain(|x| {*x != label})
        // }
        // self.hide_wins.lock().unwrap().push(label.to_string());
        // Ok(())
    // }
}

#[tauri::command]
pub async fn register_win(win_state: State<'_, WinState>, options: WinOptions)-> Result<(), WinError> {
    win_state.register(options)
}

#[tauri::command]
pub async fn new() {

}

#[tauri::command]
pub async fn open(app:  AppHandle, win: Window, label: String, args: HashMap<String, Value>, win_state: State<'_, WinState>) -> Result<&'static str, Error> {
    // 如果 label 在注册表中 则新建一个窗口，并且返回id
    // 如果是已经打开的页面 则直接打开

    // let url = format!("{}.html", name);
    // if let Some(nwin) = app.get_window(&name) {
    //     nwin.emit("open", NArgs::new(win.label(), args))?;
    //     nwin.show();
    //     Ok(nwin.label().to_string())
    // } else {
    //     let window = tauri::WindowBuilder::new(
    //         &app,
    //         name, /* the unique window label */
    //         tauri::WindowUrl::App(url.into())
    //     ).initialization_script(
    //         format!(
    //             r#"
    //                 if (window.location.origin === 'http://localhost:1420') {{
    //                     console.log("hello world from js init script");

    //                     window.__MY_CUSTOM_PROPERTY__ = '{}';
    //                 }}
    //             "#, NArgs::new(win.label(), args)).as_str()
    //     )
    //     .build()?;
    //     window.show()?;
    //     Ok(window.label().to_string())
    // }
    Ok("11")
}

#[tauri::command]
pub async fn close(win: Window, win_state: &'static WinState) -> Result<(), Error> {
    win.close()
}

#[tauri::command]
pub async fn hide(win: Window) -> Result<(), Error> {
    win.hide()
}
