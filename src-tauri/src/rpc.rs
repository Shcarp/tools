use std::{collections::{HashMap}};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NError {
    MethodNotFound,
    InvalidParams,
    InternalError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NRequest {
    pub id: String,
    pub service: String,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Clone ,Debug, Serialize, Deserialize)]
pub struct NResponse {
    pub id: String,
    pub result: Option<Value>,
    pub error: Option<NError>,
    pub message: String
}

impl NResponse {
    pub fn error(id: &str, error: NError, message: &str) -> NResponse {
        let result = Some(message.into());
        let error = Some(error);
        NResponse { id: id.to_string(), result, error, message: message.to_string() }
    }

    pub fn success(id: &str, result: Value, message: &str) -> NResponse {
        NResponse {id: id.to_string(), result: Some(result), error: None, message: message.to_string()}
    }
}

pub trait NService {
    fn call_method(&self, method: &str, args: &[Value]) -> Result<Value, NError>;
}

pub struct NServer {
    pub identify: String,
    services: HashMap<String, Box<dyn NService>>,
}

unsafe impl Send for NServer {}
unsafe impl Sync for NServer {}

impl NServer{
    pub fn new(identify: &str) -> Self {
        NServer { identify: identify.to_string(), services: HashMap::new() }
    }

    pub fn register_services(&mut self, service_name: &str ,service: Box<dyn NService>) {
        self.services.insert(service_name.to_owned(), service);
    }

    pub async fn call(&self, request: NRequest) -> Result<NResponse, ()> {
        // 查找对象
        let service = if let Some(service) = self.services.get(&request.service) {
            service
        } else {
            return Ok(NResponse::error(&request.id, NError::InvalidParams, "Service not found"));
        };

        return match service.call_method(&request.method, &request.params) {
            Ok(result) =>  Ok(NResponse::success(&request.id, result, "SUCCESS")),
            Err(NError::InternalError) => 
                Ok( 
                    NResponse::error(
                        &request.id,
                        NError::MethodNotFound,
                        "Method not found",
                    )
                ),
            Err(NError::InvalidParams) => 
                Ok(
                    NResponse::error(
                    &request.id,
                    NError::InvalidParams,
                    "Invalid params",
                )
            ),
            Err(NError::MethodNotFound) => 
                Ok(
                    NResponse::error(
                        &request.id,
                        NError::InternalError,
                        "Internal error",
                    )
                )
        }
    }
}
