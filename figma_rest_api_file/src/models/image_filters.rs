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

/// ImageFilters : Image filters to apply to the node.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFilters {
    #[serde(rename = "exposure", skip_serializing_if = "Option::is_none")]
    pub exposure: Option<f64>,
    #[serde(rename = "contrast", skip_serializing_if = "Option::is_none")]
    pub contrast: Option<f64>,
    #[serde(rename = "saturation", skip_serializing_if = "Option::is_none")]
    pub saturation: Option<f64>,
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(rename = "tint", skip_serializing_if = "Option::is_none")]
    pub tint: Option<f64>,
    #[serde(rename = "highlights", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<f64>,
    #[serde(rename = "shadows", skip_serializing_if = "Option::is_none")]
    pub shadows: Option<f64>,
}

impl ImageFilters {
    /// Image filters to apply to the node.
    pub fn new() -> ImageFilters {
        ImageFilters {
            exposure: None,
            contrast: None,
            saturation: None,
            temperature: None,
            tint: None,
            highlights: None,
            shadows: None,
        }
    }
}

