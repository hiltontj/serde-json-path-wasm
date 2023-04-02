use serde_json::Value;
use serde_json_path::{JsonPath, JsonPathExt};

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    InvalidJsonPath(#[from] serde_json_path::ParseError),
    #[error("error serializing query result: {0}")]
    SerializeQuery(#[from] serde_wasm_bindgen::Error),
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        err.to_string().into()
    }
}

#[wasm_bindgen]
pub fn parse_json(json: JsValue, path: &str) -> Result<JsValue, Error> {
    let value: Value = serde_wasm_bindgen::from_value(json)?;
    let path = JsonPath::parse(path)?;
    let query = value.json_path(&path).all();
    Ok(serde_wasm_bindgen::to_value(&query)?)
}
