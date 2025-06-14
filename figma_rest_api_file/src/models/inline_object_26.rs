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
pub struct InlineObject26 {
    /// For erroneous requests, this value is always `true`.
    #[serde(rename = "error")]
    pub error: Error,
    /// Status code
    #[serde(rename = "status")]
    pub status: Status,
    /// A string describing the error
    #[serde(rename = "message")]
    pub message: String,
}

impl InlineObject26 {
    pub fn new(error: Error, status: Status, message: String) -> InlineObject26 {
        InlineObject26 {
            error,
            status,
            message,
        }
    }
}
/// For erroneous requests, this value is always `true`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    #[serde(rename = "true")]
    True,
}

impl Default for Error {
    fn default() -> Error {
        Self::True
    }
}
/// Status code
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "500")]
    Variant500,
}

impl Default for Status {
    fn default() -> Status {
        Self::Variant500
    }
}

