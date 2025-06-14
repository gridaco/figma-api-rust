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
pub struct GradientPaint {
    /// Is the paint enabled?
    #[serde(rename = "visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    /// Overall opacity of paint (colors within the paint can also have opacity values which would blend with this)
    #[serde(rename = "opacity", skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// How this node blends with nodes behind it in the scene
    #[serde(rename = "blendMode")]
    pub blend_mode: models::BlendMode,
    /// The string literal representing the paint's type. Always check the `type` before reading other properties.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This field contains three vectors, each of which are a position in normalized object space (normalized object space is if the top left corner of the bounding box of the object is (0, 0) and the bottom right is (1,1)). The first position corresponds to the start of the gradient (value 0 for the purposes of calculating gradient stops), the second position is the end of the gradient (value 1), and the third handle position determines the width of the gradient.
    #[serde(rename = "gradientHandlePositions")]
    pub gradient_handle_positions: Vec<models::Vector>,
    /// Positions of key points along the gradient axis with the colors anchored there. Colors along the gradient are interpolated smoothly between neighboring gradient stops.
    #[serde(rename = "gradientStops")]
    pub gradient_stops: Vec<models::ColorStop>,
}

impl GradientPaint {
    pub fn new(blend_mode: models::BlendMode, r#type: Type, gradient_handle_positions: Vec<models::Vector>, gradient_stops: Vec<models::ColorStop>) -> GradientPaint {
        GradientPaint {
            visible: None,
            opacity: None,
            blend_mode,
            r#type,
            gradient_handle_positions,
            gradient_stops,
        }
    }
}
/// The string literal representing the paint's type. Always check the `type` before reading other properties.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "GRADIENT_LINEAR")]
    GradientLinear,
    #[serde(rename = "GRADIENT_RADIAL")]
    GradientRadial,
    #[serde(rename = "GRADIENT_ANGULAR")]
    GradientAngular,
    #[serde(rename = "GRADIENT_DIAMOND")]
    GradientDiamond,
}

impl Default for Type {
    fn default() -> Type {
        Self::GradientLinear
    }
}

