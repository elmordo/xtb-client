use serde_json::Value;
use thiserror::Error;
use crate::api::{ErrorResponse, Response};

pub trait XtbConnection {
    fn send_command(&mut self, command: &str, payload: Option<Value>) -> Result<Response, ErrorResponse>;
}


#[derive(Debug, Error)]
pub enum XtbConnectionError {
    #[error("Cannot connect to server ({0}, port: {1})")]
    CannotConnect(String, u16)
}


pub struct BasicXtbConnection {

}


impl BasicXtbConnection {
    pub fn new(host: &str, port: u16) -> Result<Self, XtbConnectionError> {
        todo!()
    }
}


impl XtbConnection for BasicXtbConnection {
    fn send_command(&mut self, command: &str, payload: Option<Value>) -> Result<Response, ErrorResponse> {
        todo!()
    }
}
