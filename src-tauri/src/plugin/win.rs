#[cfg(target_os="windows")]
use rdev::{grab};
#[cfg(target_os="macos")]
use crate::utils::mac_keyboard_event;
#[cfg(target_os="macos")]
use core_graphics::event::EventField;

use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::marker::PhantomPinned;
use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{collections::HashMap, fmt::Display};
use tauri::async_runtime::Mutex;
use tauri::{plugin::Plugin, AppHandle, Invoke, Manager, Runtime, State, Window};


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
    HideWindowFail(String, String),
}

#[derive(Serialize, Deserialize)]
struct NArgs {
    send: String,
    args: HashMap<String, Value>,
}

impl NArgs {
    pub fn new(send: &str, args: HashMap<String, Value>) -> String {
        NArgs {
            send: send.to_string(),
            args,
        }
        .to_string()
    }
}

impl Display for NArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json_str = serde_json::to_string(self).unwrap();
        write!(f, "{}", json_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WinOptions {
    pub win_type: String,
    pub url: String,
    #[serde(default)]
    pub overopen: Option<bool>,
    #[serde(default)]
    pub position: Option<(f64, f64)>,
    #[serde(default)]
    pub size: Option<(f64, f64)>,
    #[serde(default)]
    pub min_size: Option<(f64, f64)>,
    #[serde(default)]
    pub max_size: Option<(f64, f64)>,
    #[serde(default)]
    pub resizable: Option<bool>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub fullscreen: Option<bool>,
    #[serde(default)]
    pub focus: Option<bool>,
    #[serde(default)]
    pub center: Option<bool>,
}

impl WinOptions {
    pub fn new(label: String, url: String) -> Self {
        Self {
            win_type: label,
            url: url,
            overopen: None,
            position: None,
            size: None,
            min_size: None,
            max_size: None,
            resizable: None,
            title: None,
            fullscreen: None,
            focus: None,
            center: None,
        }
    }
}

#[derive(Debug)]
pub struct WinState {
    pub register_win_types: HashMap<String, WinOptions>,
    pub open_wins: Vec<String>,
    pub hide_wins: Vec<String>,
    _marker: PhantomPinned,
}

unsafe impl Send for WinState {}
unsafe impl Sync for WinState {}

impl WinState {
    pub fn new() -> Self {
        Self {
            register_win_types: HashMap::new(),
            open_wins: vec![],
            hide_wins: vec![],
            _marker: PhantomPinned,
        }
    }

    fn is_open(&self, label: &str) -> bool {
        self.open_wins
            .iter()
            .find(|&win| *win == label.to_string())
            .is_some()
    }
    fn is_hide(&self, label: &str) -> bool {
        self.hide_wins
            .iter()
            .find(|&win| *win == label.to_string())
            .is_some()
    }
    fn is_register(&self, win_type: &str) -> bool {
        self.register_win_types.get(win_type).is_some()
    }

    pub fn get_options_by_type(&self, win_type: &str) -> Option<&WinOptions> {
        self.register_win_types.get(win_type)
    }

    pub fn register(&mut self, options: WinOptions) -> Result<(), WinError> {
        if self.is_register(&options.win_type) {
            return Err(WinError::AlreadyExist(format!(
                "Win type {}",
                &options.win_type
            )));
        }
        self.register_win_types
            .insert(options.win_type.clone(), options);
        Ok(())
    }

    pub fn open(&mut self, label: &str) -> Result<(), WinError> {
        if self.is_open(label) {
            return Err(WinError::AlreadyOpen(label.to_string()));
        }
        if self.is_hide(label) {
            self.hide_wins.retain(|x| *x != label)
        }
        self.open_wins.push(label.to_string());
        Ok(())
    }

    pub fn hide(&mut self, label: &str) -> Result<(), WinError> {
        if self.is_hide(label) {
            return Err(WinError::AlreadyHide(label.to_string()));
        }
        if self.is_open(label) {
            self.open_wins.retain(|x| *x != label)
        }
        self.hide_wins.push(label.to_string());
        Ok(())
    }

    pub fn close(&mut self, label: &str) -> Result<(), WinError> {
        if self.is_hide(label) {
            self.hide_wins.retain(|x| *x != label)
        }
        if self.is_open(label) {
            self.open_wins.retain(|x| *x != label)
        }
        Ok(())
    }


}

fn new_window<R: Runtime>(
    app: &AppHandle<R>,
    open: &str,
    label: &str,
    options: &WinOptions,
    args: HashMap<String, Value>,
) -> Result<(), WinError> {
    let mut window = tauri::WindowBuilder::new(
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
    );

    if let Some(position) = options.position {
        window = window.position(position.0, position.1);
    }

    if let Some(size) = options.size {
        window = window.inner_size(size.0, size.1);
    }

    if let Some(max_size) = options.max_size {
        window = window.max_inner_size(max_size.0, max_size.1);
    }

    if let Some(min_inner_size) = options.max_size {
        window = window.max_inner_size(min_inner_size.0, min_inner_size.1);
    }

    if let Some(ref title) = options.title {
        window = window.title(title);
    }

    if let Some(focused) = options.focus {
        window = window.focused(focused);
    }

    if let Some(fullscreen) = options.fullscreen {
        window = window.fullscreen(fullscreen);
    }

    if let Some(resizable) = options.resizable {
        window = window.resizable(resizable);
    }

    if let Some(center) = options.center {
        if center {
            window = window.center();
        }
    }

    let window = window.build().or_else(|error| {
        error!("{} build error: {} ", label.to_string(), error);
        Err(WinError::OpenWindowFail(
            label.to_string(),
            "win build error".to_string(),
        ))
    })?;

    window.show().or_else(|error| {
        error!("{} build error: {} ", label.to_string(), error);
        Err(WinError::OpenWindowFail(
            label.to_string(),
            "win build error".to_string(),
        ))
    })?;
    Ok(())
}

#[tauri::command]
async fn open<R: Runtime>(
    app: AppHandle<R>,
    win: Window<R>,
    label: &str,
    args: HashMap<String, Value>,
    win_state: State<'_, Arc<Mutex<WinState>>>,
) -> Result<String, WinError> {
    // 如果 label 在注册中 则新建一个窗口，并且返回id
    // 如果是已经打开的页面 则直接打开
    let mut win_state = win_state.lock().await;
    if !win_state.is_register(label) {
        return Err(WinError::NotRegister(label.to_string()));
    }
    let options = win_state.get_options_by_type(label).unwrap();
    if let Some(ref value) = options.overopen {
        if *value {
            let win_label = format!("{}_{}", label, WIN_COUNT.fetch_add(1, Ordering::Relaxed));
            new_window(&app, win.label(), &win_label, options, args)?;
            win_state.open(label)?;
            return Ok(win_label);
        }
    }

    if let Some(twin) = app.get_window(label) {
        twin.emit("open", NArgs::new(win.label(), args))
            .or_else(|_| {
                Err(WinError::OpenWindowFail(
                    label.to_string(),
                    "args error".to_string(),
                ))
            })?;
        win_state.open(label)?;
    } else {
        new_window(&app, win.label(), label, options, args)?;
        win_state.open(label)?;
    }
    return Ok(label.to_string());
}

#[tauri::command]
async fn close<R: Runtime>(
    app: AppHandle<R>,
    win: Window<R>,
    win_state: State<'_, Arc<Mutex<WinState>>>,
    label: &str,
) -> Result<(), WinError> {
    let mut win_state = win_state.lock().await;
    if label == "" {
        win.close().or_else(|error| {
            println!("{}", error.to_string());
            Err(WinError::CloseWindowFail(
                win.label().to_string(),
                error.to_string(),
            ))
        })?;
        win_state.close(win.label())?;
    } else {
        if let Some(twin) = app.get_window(label) {
            twin.close().or_else(|error| {
                println!("{}", error.to_string());
                Err(WinError::CloseWindowFail(
                    label.to_string(),
                    error.to_string(),
                ))
            })?;
            // 从win_state 中删除
            win_state.close(label)?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn hide<R: Runtime>(
    app: AppHandle<R>,
    win: Window<R>,
    win_state: State<'_, Arc<Mutex<WinState>>>,
    label: &str,
) -> Result<(), WinError> {
    let mut win_state = win_state.lock().await;
    if label == "" {
        if win_state.is_hide(win.label()) {
            return Err(WinError::HideWindowFail(
                win.label().to_string(),
                "already hide".to_string(),
            ));
        }
        win.hide().or_else(|error| {
            println!("{}", error.to_string());
            Err(WinError::HideWindowFail(
                win.label().to_string(),
                error.to_string(),
            ))
        })?;
        win_state.hide(win.label())?;
    } else {
        if win_state.is_hide(win.label()) {
            return Err(WinError::HideWindowFail(
                label.to_string(),
                "already hide".to_string(),
            ));
        }
        if let Some(twin) = app.get_window(label) {
            twin.hide().or_else(|error| {
                println!("{}", error.to_string());
                Err(WinError::HideWindowFail(
                    label.to_string(),
                    error.to_string(),
                ))
            })?;
            // 从win_state 中删除
            win_state.hide(label)?;
        }
        {
            panic!("错误, hide 已经关闭的窗口")
        }
    }
    Ok(())
}

pub struct NWindowsPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
    win_state: Arc<Mutex<WinState>>
}

unsafe impl<R: Runtime> Send for NWindowsPlugin<R> {}
unsafe impl<R: Runtime> Sync for NWindowsPlugin<R> {}

impl<R: Runtime> NWindowsPlugin<R> {
    // you can add configuration fields here,·
    // see https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![open, close, hide]),
            win_state: Arc::new(Mutex::new(WinState::new()))
        }
    }
}

impl<R: Runtime> Plugin<R> for NWindowsPlugin<R> {
    fn name(&self) -> &'static str {
        "win"
    }

    fn initialize(&mut self, app: &AppHandle<R>, config: Value) -> tauri::plugin::Result<()> {
        let config: Vec<WinOptions> = serde_json::from_value(config)?;
        info!("Page Config {:?}", config);

        config.iter().for_each(|options| {
            self.win_state.blocking_lock().register(options.clone()).expect("请页面配置");
        });
        app.manage(self.win_state.clone());
        #[cfg(target_os="windows")]
        tauri::async_runtime::spawn(async move {
            rdev::grab(move |event| {
                let is_block: bool = match event.event_type {
                    rdev::EventType::KeyPress(key) => {
                        match key {
                            rdev::Key::Alt => {
                                info!("按了{:?} ALT ALT", key);
                                true
                            },
                            rdev::Key::KeyU => {
                                info!("按了{:?} UUUUUUU", key);
                                true
                            },
                            _ => {
                                info!("{:?}", key);
                                false
                            }
                        }
                    },
                    rdev::EventType::MouseMove { x, y } => {
                        info!("x:{}, y: {}", x, y);
                        false
                    }
                    _ => {
                        info!("啥也没有");
                        false
                    },
                };
                if is_block {
                    None
                } else {
                    Some(event)
                }
            })
        });
        #[cfg(target_os="macos")]
        mac_keyboard_event(|event| {
            println!("{:?}", event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE));
            println!("{:?}", event.get_flags());
        });
        info!("load success");
        Ok(())
    }

    fn initialization_script(&self) -> Option<String> {
    None
  }

    fn created(&mut self, window: Window<R>) {
        info!("create {}", window.label())
    }

    fn on_page_load(&mut self, window: Window<R>, payload: tauri::PageLoadPayload) {
        
    }

    fn on_event(&mut self, app: &AppHandle<R>, event: &tauri::RunEvent) {
        match event {
            tauri::RunEvent::Exit => {
                info!("Application EXIT")
            },
            tauri::RunEvent::WindowEvent { label, event , .. } => {
                
            },
            // tauri::RunEvent::Ready => {
            //     info!("Application ready");
            //     let options = self.win_state
            //         .blocking_lock()
            //         .get_options_by_type("main")
            //         .expect("not found main page")
            //         .clone();
            //     let handle = app.app_handle();
            //     std::thread::spawn(move || {
            //         new_window(
            //             &handle,
            //              "", 
            //              &options.win_type, 
            //              &options, 
            //              HashMap::new()
            //         ).expect("open first page error");
            //         let c_win_state: State<'_, Arc<Mutex<WinState>>> = handle.state();
            //         c_win_state.blocking_lock().open("main").expect("open first page error");
            //         info!("open main success")
            //     });
            // },
            tauri::RunEvent::Resumed => {

            },
            tauri::RunEvent::MainEventsCleared => {

            },
            _ => {},
        }
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}

