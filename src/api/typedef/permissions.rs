use json::{JsonValue, object};

use crate::api::typedef::{BackendError, jsonutils::SerializableJson};

pub struct Permission {
    pub permission: Box<str>,
    pub value: bool
}

pub struct Group {
    pub name: Box<str>,
    pub prefix: Box<str>,
    pub suffix: Box<str>,
    pub perms: Vec<Permission>,
}

impl Group {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), prefix: "".into(), suffix: "".into(), perms: vec![] }
    }
}

impl SerializableJson for Permission {
    fn to_json(&self) -> json::JsonValue {
        object! {
            permission: self.permission.as_ref(),
            value: self.value
        }
    }

    fn from_json(json: &json::JsonValue) -> Result<Self, super::BackendError> where Self: Sized {
        Ok(Self {
            permission: json["permission"].as_str().ok_or(BackendError::new("permission.permission missing", 400))?.into(),
            value: json["value"].as_bool().ok_or(BackendError::new("permission.value missing", 400))?,
        })
    }
}

impl SerializableJson for Group {
    fn to_json(&self) -> json::JsonValue {
        object! {
            name: self.name.as_ref(),
            prefix: self.prefix.as_ref(),
            suffix: self.suffix.as_ref(),
            perms: JsonValue::Array(
                self.perms.iter().map(|p| p.to_json()).collect()
            )
        }
    }

    fn from_json(json: &json::JsonValue) -> Result<Self, super::BackendError> where Self: Sized {
        Ok(Self {
            name: json["name"].as_str().ok_or(BackendError::new("group.name missing", 400))?.into(),
            prefix: json["prefix"].as_str().ok_or(BackendError::new("group.prefix missing", 400))?.into(),
            suffix: json["suffix"].as_str().ok_or(BackendError::new("group.suffix missing", 400))?.into(),
            perms: json["perms"].members().filter_map(|j| Permission::from_json(j).ok()).collect()
        })
    }
}
