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

/// Vector : A 2d vector.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    /// X coordinate of the vector.
    #[serde(rename = "x")]
    pub x: f64,
    /// Y coordinate of the vector.
    #[serde(rename = "y")]
    pub y: f64,
}

impl Vector {
    /// A 2d vector.
    pub fn new(x: f64, y: f64) -> Vector {
        Vector {
            x,
            y,
        }
    }
}

