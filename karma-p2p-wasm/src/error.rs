use wasm_bindgen::JsValue;

pub enum Error {
    ErrAddrType,
    WebsysError(JsValue),
    SerdeError(serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeError(e)
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Error::WebsysError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
