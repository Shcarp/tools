use std::{collections::HashMap, fmt::Display};
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Window, State, Manager};
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};

use tauri::{ async_runtime::Mutex };

static WIN_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, thiserror::Error, Serialize)]
pub enum WinError {
  #[error("Options `{0}` is not find")]
  MissOptions(String),
  #[error("{0} Already exist")]
  AlreadyExist(String),
  #[error("{0} Already open")]
  AlreadyOpen(String),
  #[error("{0} Already hide")]
  AlreadyHide(String),
  #[error("{0} Not register")]
  NotRegister(String),
  #[error("{0} open fail: {1}")]
  OpenWindowFail(String, String),
  #[error("{0} close faill: {1}")]
  CloseWindowFail(String, String),
  #[error("{0} hide faill: {1}")]
  HideWindowFail(String, String)
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
    pub position: (f64, f64),
    pub height: f64,
    pub width: f64,
    pub min_width: f64,
    pub min_height: f64,
    pub max_width: f64,
    pub max_height: f64,
    pub resizable: bool,
    pub title: String,
    pub fullscreen: bool,
    pub focus: bool
}

pub struct WinState {
    pub register_win_types: HashMap<String, WinOptions>,
    pub open_wins: Vec<String>,
    pub hide_wins: Vec<String>
}

unsafe impl Send for WinState {}
unsafe impl Sync for WinState {}

impl WinState {
    pub fn new() -> Self {
        Self {
            register_win_types: HashMap::new(),
            open_wins: vec![] ,
            hide_wins: vec![]
        }
    }

    fn is_open(&self, label: &str) -> bool {
        self.open_wins.iter().find(|&win| {*win == label.to_string()}).is_some()
    }
    fn is_hide(&self, label: &str) -> bool {
        self.hide_wins.iter().find(|&win| {*win == label.to_string()}).is_some()
    }
    fn is_register(&self, win_type: &str) -> bool {
        self.register_win_types.get(win_type).is_some()
    }

    pub fn get_options_by_type(&self, win_type: &str) -> Option<&WinOptions> {
        self.register_win_types.get(win_type)
    }

    pub fn register(&mut self, options: WinOptions)-> Result<(), WinError> {
        if self.is_register(&options.win_type) {
            return Err(WinError::AlreadyExist(format!("Win type {}", &options.win_type)))
        }
        self.register_win_types.insert(options.win_type.clone(), options);
        Ok(())
    }

    pub fn open(&mut self, label: &str)-> Result<(), WinError> {
        if self.is_open(label) {
            return Err(WinError::AlreadyOpen(label.to_string()));
        }
        if self.is_hide(label) {
            self.hide_wins.retain(|x| {*x != label})
        }
        self.open_wins.push(label.to_string());
        Ok(())
    }

    pub fn hide(&mut self, label: &str)-> Result<(), WinError> {
        if self.is_hide(label) {
            return Err(WinError::AlreadyHide(label.to_string()));
        }
        if self.is_open(label) {
            self.open_wins.retain(|x| {*x != label})
        }
        self.hide_wins.push(label.to_string());
        Ok(())
    }

    pub fn close(&mut self, label: &str)-> Result<(), WinError> {
        if self.is_hide(label) {
            self.hide_wins.retain(|x| {*x != label})
        }
        if self.is_open(label) {
            self.open_wins.retain(|x| {*x != label})
        }
        Ok(())
    }
}

#[tauri::command]
pub async fn register_win(win_state:  State<'_, Mutex<WinState>>, options: WinOptions)-> Result<(), WinError> {
    let mut win_state = win_state.lock().await;
    win_state.register(options)
}

fn new_window(app: &AppHandle, open: &str, label: &str ,options: &WinOptions, args: HashMap<String, Value>) -> Result<(), WinError> {
    let window = tauri::WindowBuilder::new(
        app,
        label, /* the unique window label */
        tauri::WindowUrl::App(options.url.to_owned().into())
    ).initialization_script(
        format!(
            r#"
                if (window.location.origin === 'http://localhost:1420' || window.location.origin === '') {{
                    console.log("init");

                    window.__MY_CUSTOM_PROPERTY__ = '{}';
                }}
            "#, NArgs::new(open, args)).as_str()
    )
    .position(options.position.0, options.position.1)
    .title(options.title.clone())
    .focused(options.focus)
    .fullscreen(options.fullscreen)
    .inner_size(options.width, options.height)
    .max_inner_size(options.max_width, options.max_height)
    .min_inner_size(options.min_width, options.min_height)
    .resizable(options.resizable)
    .build()
        .or_else(|error| {
        println!("{}", error.to_string());
        Err(WinError::OpenWindowFail(label.to_string(), "win build error".to_string()))
    })?;

    window.show().or_else(|error| {
        println!("{}", error.to_string());
        Err(WinError::OpenWindowFail(label.to_string(), "win build error".to_string()))
    })?;
    Ok(())
}

#[tauri::command]
pub async fn open(app: AppHandle, win: Window, label: &str, args: HashMap<String, Value>, win_state: State<'_, Mutex<WinState>>) -> Result<String, WinError> {
    // 如果 label 在注册中 则新建一个窗口，并且返回id
    // 如果是已经打开的页面 则直接打开
    let mut win_state = win_state.lock().await;
    if !win_state.is_register(label) {
        return Err(WinError::NotRegister(label.to_string()));
    }
    let options = win_state.get_options_by_type(label).unwrap();
    if options.overopen {
        let win_label = format!("{}_{}", label, WIN_COUNT.fetch_add(1, Ordering::Relaxed));
        new_window(&app, win.label(), &win_label, options, args)?;
        win_state.open(label)?;
        return Ok(win_label);
    }

    if let Some(twin) = app.get_window(label) {
        if win_state.is_open(label) {
            twin.emit("open", NArgs::new(win.label(), args))
                .or_else(|_| {
                     Err(WinError::OpenWindowFail(label.to_string(), "args error".to_string()))
                    })?;
        } else if win_state.is_hide(label) {
            twin.emit("open", NArgs::new(win.label(), args))
                .or_else(|_| { 
                    Err(WinError::OpenWindowFail(label.to_string(), "args error".to_string()))
                })?;
                twin.show().or_else(|_| {Err(WinError::OpenWindowFail(label.to_string(), "show error".to_string()))})?;
                win_state.open(label)?;
        }
    } else {
        new_window(&app, win.label(), label, options, args)?;
    }
    return Ok(label.to_string());
}

#[tauri::command]
pub async fn close(app: AppHandle, win: Window, win_state: State<'_, Mutex<WinState>>, label: &str) -> Result<(), WinError> {
    let mut win_state = win_state.lock().await;
    if label == "" {
        win.close().or_else(|error| {
            println!("{}", error.to_string());
            Err(WinError::CloseWindowFail(win.label().to_string(), error.to_string()))
        })?;
        win_state.close(win.label())?;
    } else {
        if let Some(twin) = app.get_window(label) {
            twin.close().or_else(|error| {
                println!("{}", error.to_string());
                Err(WinError::CloseWindowFail(label.to_string(), error.to_string()))
            })?;
            // 从win_state 中删除
            win_state.close(label)?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn hide(app: AppHandle, win: Window, win_state: State<'_, Mutex<WinState>>, label: &str) -> Result<(), WinError> {
    let mut win_state = win_state.lock().await;
    if label == "" {
        if win_state.is_hide(win.label()) {
            return  Err(WinError::HideWindowFail(win.label().to_string(), "already hide".to_string()));
        }
        win.hide().or_else(|error| {
            println!("{}", error.to_string());
            Err(WinError::HideWindowFail(win.label().to_string(), error.to_string()))
        })?;
        win_state.hide(win.label())?;
    } else {
        if win_state.is_hide(win.label()) {
            return  Err(WinError::HideWindowFail(label.to_string(), "already hide".to_string()));
        }
        if let Some(twin) = app.get_window(label) {
            twin.hide().or_else(|error| {
                println!("{}", error.to_string());
                Err(WinError::HideWindowFail(label.to_string(), error.to_string()))
            })?;
            // 从win_state 中删除
            win_state.hide(label)?;
        } {
            panic!("错误, hide 已经关闭的窗口")
        }
    }
    Ok(())
}
