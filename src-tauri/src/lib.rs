pub mod command;
pub mod plugin;
pub mod rpc;
pub mod utils;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::command::{greet, recive_message};
pub use crate::win::{WinOptions, WinState};
use log::{error, info, LevelFilter};
use plugin::win;
use rpc::{NResponse, NServer, NService};
use tauri::{Manager, State, Wry};
use tauri_plugin_log::LogTarget;

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
    fn call_method(
        &self,
        method: &str,
        args: &[serde_json::Value],
    ) -> Result<serde_json::Value, rpc::NError> {
        match method {
            "height" => Ok(self.height.into()),
            "width" => Ok(self.width.into()),
            JUDAGE_SERVICE => Ok("".into()),
            _ => Err(rpc::NError::MethodNotFound),
        }
    }
}

impl NApp {
    pub fn new() -> NApp {
        let mut server = NServer::new("我是谁");
        // 在这里注册服务（对象）
        server.register_services(
            "test",
            Box::new(TestRpc {
                width: 30,
                height: 40,
            }),
        );

        let builder = tauri::Builder::default()
            .plugin(
                tauri_plugin_log::Builder::default()
                    .targets([
                        // LogTarget::LogDir,
                        LogTarget::Stdout,
                        LogTarget::Webview,
                        LogTarget::Folder("log".into()),
                    ])
                    .level(LevelFilter::Debug)
                    .build(),
            )
            .plugin(win::NWindowsPlugin::new())
            .setup(|app_handle| {
                let app = app_handle.app_handle();
                app_handle.listen_global("show", move |event| {
                    info!("{:?}", event);
                    match app.get_window("main") {
                        Some(win) => {
                            if let Err(_) = win.show() {
                                info!("open window error");
                                // 弹窗提醒
                            }
                        }
                        None => {
                            let win_state: State<'_, Arc<Mutex<WinState>>> = app.state();
                            let options = match win_state
                                .lock()
                                .expect("mutex poisoned")
                                .get_options_by_type("main")
                            {
                                Some(options) => options.clone(),
                                None => {
                                    error!("not find main page options");
                                    // 弹窗
                                    panic!()
                                }
                            };

                            win::creator_new_window(
                                &app,
                                "",
                                &options.win_type,
                                &options,
                                HashMap::new(),
                            )
                            .or_else(|error| {
                                error!("creator window error: {}", error);
                                Err("open win error")
                            })
                            .unwrap()
                            .show()
                            .or_else(|error| {
                                error!("show window error: {}", error);
                                Err("open win error")
                            })
                            .unwrap();
                        }
                    }
                });
                Ok(())
            })
            .manage(server)
            .invoke_handler(tauri::generate_handler![greet, recive_message]);
        NApp { builder }
    }

    pub fn run(self) {
        self.builder.build(tauri::generate_context!()).unwrap().run(
            |_app_handle, event| match event {
                tauri::RunEvent::ExitRequested { api, .. } => {
                    api.prevent_exit();
                }
                _ => {}
            },
        );
    }

    pub fn register_module<T>(mut self, state: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.builder = self.builder.manage(state);
        return self;
    }
}

pub fn lmian() {
    NApp::new().run()
}
