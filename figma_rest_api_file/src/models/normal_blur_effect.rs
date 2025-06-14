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
pub struct NormalBlurEffect {
    /// A string literal representing the effect's type. Always check the type before reading other properties.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Whether this blur is active.
    #[serde(rename = "visible")]
    pub visible: bool,
    /// Radius of the blur effect
    #[serde(rename = "radius")]
    pub radius: f64,
    #[serde(rename = "boundVariables", skip_serializing_if = "Option::is_none")]
    pub bound_variables: Option<Box<models::BaseBlurEffectBoundVariables>>,
    /// The string literal 'NORMAL' representing the blur type. Always check the blurType before reading other properties.
    #[serde(rename = "blurType", skip_serializing_if = "Option::is_none")]
    pub blur_type: Option<BlurType>,
}

impl NormalBlurEffect {
    pub fn new(r#type: Type, visible: bool, radius: f64) -> NormalBlurEffect {
        NormalBlurEffect {
            r#type,
            visible,
            radius,
            bound_variables: None,
            blur_type: None,
        }
    }
}
/// A string literal representing the effect's type. Always check the type before reading other properties.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "LAYER_BLUR")]
    LayerBlur,
    #[serde(rename = "BACKGROUND_BLUR")]
    BackgroundBlur,
}

impl Default for Type {
    fn default() -> Type {
        Self::LayerBlur
    }
}
/// The string literal 'NORMAL' representing the blur type. Always check the blurType before reading other properties.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BlurType {
    #[serde(rename = "NORMAL")]
    Normal,
}

impl Default for BlurType {
    fn default() -> BlurType {
        Self::Normal
    }
}

