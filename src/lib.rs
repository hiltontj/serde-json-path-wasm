use serde::Serialize;
use serde_json::Value;
use serde_json_path::JsonPath as JsonPathInternal;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Error type for query parsing or JSON serialization/deserialization
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// There was a problem while parsing the JSONPath query
    #[error(transparent)]
    InvalidJsonPath(#[from] serde_json_path::ParseError),
    /// There was a problem while serdializing/deserializing JSON
    #[error("error serializing query result: {0}")]
    SerializeQuery(#[from] serde_wasm_bindgen::Error),
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        err.to_string().into()
    }
}

/// Holds a parsed and valid JSONPath query
#[wasm_bindgen]
pub struct JsonPath(JsonPathInternal);

#[wasm_bindgen]
impl JsonPath {
    /// Create a [`JsonPath`] by parsing a JSONPath query string
    pub fn parse(path: &str) -> Result<JsonPath, JsError> {
        Ok(JsonPath(JsonPathInternal::parse(path)?))
    }

    /// Query a JSON value to produce a list of nodes
    pub fn query(&self, json: JsValue) -> Result<JsValue, JsError> {
        let value: Value = serde_wasm_bindgen::from_value(json)?;
        let nodes = self.0.query(&value);
        Ok(nodes.serialize(&Serializer::json_compatible())?)
    }
}
