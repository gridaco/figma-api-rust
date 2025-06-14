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

/// ExportSetting : An export setting.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportSetting {
    #[serde(rename = "suffix")]
    pub suffix: String,
    #[serde(rename = "format")]
    pub format: Format,
    #[serde(rename = "constraint")]
    pub constraint: Box<models::Constraint>,
}

impl ExportSetting {
    /// An export setting.
    pub fn new(suffix: String, format: Format, constraint: models::Constraint) -> ExportSetting {
        ExportSetting {
            suffix,
            format,
            constraint: Box::new(constraint),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "JPG")]
    Jpg,
    #[serde(rename = "PNG")]
    Png,
    #[serde(rename = "SVG")]
    Svg,
    #[serde(rename = "PDF")]
    Pdf,
}

impl Default for Format {
    fn default() -> Format {
        Self::Jpg
    }
}

