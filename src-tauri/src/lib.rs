pub mod rpc;
pub mod command;
pub mod win;

use rpc::{NServer, NResponse, NService};
use tauri::{ Wry, async_runtime::Mutex, Manager };
use crate::command::{greet, recive_message};
pub use crate::win::{open, WinState};

const JUDAGE_SERVICE: &str = "JUDAGE_SERVICE";

#[derive(Clone, serde::Serialize)]
pub struct Payload {
  message: NResponse,
}

pub struct NApp {
    builder: tauri::Builder<Wry>,
}

struct TestRpc {
    height: i32,
    width: i32,
}

impl NService for TestRpc {
    fn call_method(&self, method: &str, args: &[serde_json::Value]) -> Result<serde_json::Value, rpc::NError> {
        match method {
            "height" => Ok(self.height.into()),
            "width" => Ok(self.width.into()),
            JUDAGE_SERVICE => Ok("".into()),
            _ => Err(rpc::NError::MethodNotFound)
        }
    }
}

impl NApp {
    pub fn new() -> NApp {
        let mut server = NServer::new("我是谁");
        // 在这里注册服务（对象）
        server.register_services("test", Box::new(TestRpc {width: 30, height: 40}));

        let builder = tauri::Builder::default()
            .setup(|app| {

                // app.listen_global("", |event| {
                //     println!("got event-name with payload {:?}", event.payload());
                //   });

                Ok(())
            })
            .manage(server)
            .invoke_handler(tauri::generate_handler![greet, recive_message, open]);
        NApp { builder }
    }

    pub fn run(self) {
        self.builder
            .build(tauri::generate_context!())
            .unwrap()
            .run(|_app_handle, event | match event {
                tauri::RunEvent::ExitRequested { api, .. } => {
                    api.prevent_exit();
                }
                tauri::RunEvent::WindowEvent { label, event, .. } => {
                    match event {
                        tauri::WindowEvent::Resized(_) => todo!(),
                        tauri::WindowEvent::Moved(_) => todo!(),
                        tauri::WindowEvent::CloseRequested { api , .. } => todo!(),
                        tauri::WindowEvent::Destroyed => todo!(),
                        tauri::WindowEvent::Focused(_) => todo!(),
                        tauri::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size , .. } => todo!(),
                        tauri::WindowEvent::FileDrop(_) => todo!(),
                        tauri::WindowEvent::ThemeChanged(_) => todo!(),
                        _ => todo!(),
                    }
                }
                _ => {}
            });
    }

    pub fn register_module<T>(mut self, state: T) -> Self 
        where T: Send + Sync + 'static  
    {
        self.builder = self.builder.manage(state);
        return self;
    }
}

pub fn lmian() {
    let mut winstate = WinState::new();
    NApp::new().register_module(Mutex::new(winstate)).run()
}
