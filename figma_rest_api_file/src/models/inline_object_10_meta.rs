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
pub struct InlineObject10Meta {
    #[serde(rename = "styles")]
    pub styles: Vec<models::PublishedStyle>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Box<models::ResponseCursor>>,
}

impl InlineObject10Meta {
    pub fn new(styles: Vec<models::PublishedStyle>) -> InlineObject10Meta {
        InlineObject10Meta {
            styles,
            cursor: None,
        }
    }
}

