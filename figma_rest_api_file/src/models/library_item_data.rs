/*
 * Figma API
 *
 * This is the OpenAPI specification for the [Figma REST API](https://www.figma.com/developers/api).  Note: we are releasing the OpenAPI specification as a beta given the large surface area and complexity of the REST API. If you notice any inaccuracies with the specification, please [file an issue](https://github.com/figma/rest-api-spec/issues).
 *
 * The version of the OpenAPI document: 0.31.0
 * Contact: support@figma.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LibraryItemData : An object representing the library item information in the payload of the `LIBRARY_PUBLISH` event
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryItemData {
    /// Unique identifier for the library item
    #[serde(rename = "key")]
    pub key: String,
    /// Name of the library item
    #[serde(rename = "name")]
    pub name: String,
}

impl LibraryItemData {
    /// An object representing the library item information in the payload of the `LIBRARY_PUBLISH` event
    pub fn new(key: String, name: String) -> LibraryItemData {
        LibraryItemData {
            key,
            name,
        }
    }
}

