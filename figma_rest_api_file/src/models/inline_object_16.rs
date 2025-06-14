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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject16 {
    /// Status code
    #[serde(rename = "status")]
    pub status: Status,
    /// A string describing the error
    #[serde(rename = "err")]
    pub err: String,
}

impl InlineObject16 {
    pub fn new(status: Status, err: String) -> InlineObject16 {
        InlineObject16 {
            status,
            err,
        }
    }
}
/// Status code
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "400")]
    Variant400,
}

impl Default for Status {
    fn default() -> Status {
        Self::Variant400
    }
}

