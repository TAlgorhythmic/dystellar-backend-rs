use json::JsonValue;

use crate::api::typedef::BackendError;

pub trait SerializableJson {
    fn to_json(&self) -> JsonValue;
    fn from_json(json: &JsonValue) -> Result<Self, BackendError> where Self: Sized;
}
