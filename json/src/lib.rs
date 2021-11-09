use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_json::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson<'a> {
    pub private: Option<bool>,
    pub license: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,

    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<&'a RawValue>,
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<&'a RawValue>,

    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<&'a RawValue>,

    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserslist: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub david: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_dependencies: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jest: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jest_sonar: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licence: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "lint-staged", deserialize = "lint-staged"))]
    pub lintstaged: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_dependencies: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<HashMap<&'a str, &'a str>>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_config: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolutions: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_effects: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typings: Option<&'a RawValue>,
    
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<&'a RawValue>,
}


impl<'a> PackageJson<'a> {
    pub fn load<T: Deserialize<'a>>(data: &'a str) -> Result<T> {
        serde_json::from_str(data)
    }

    pub fn save(json: &Self) -> Result<String> {
        serde_json::to_string_pretty(json)
    }
}