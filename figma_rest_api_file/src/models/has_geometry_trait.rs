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
pub struct HasGeometryTrait {
    /// An array of fill paints applied to the node.
    #[serde(rename = "fills")]
    pub fills: Vec<models::Paint>,
    /// A mapping of a StyleType to style ID (see Style) of styles present on this node. The style ID can be used to look up more information about the style in the top-level styles field.
    #[serde(rename = "styles", skip_serializing_if = "Option::is_none")]
    pub styles: Option<std::collections::HashMap<String, String>>,
    /// An array of stroke paints applied to the node.
    #[serde(rename = "strokes", skip_serializing_if = "Option::is_none")]
    pub strokes: Option<Vec<models::Paint>>,
    /// The weight of strokes on the node.
    #[serde(rename = "strokeWeight", skip_serializing_if = "Option::is_none")]
    pub stroke_weight: Option<f64>,
    /// Position of stroke relative to vector outline, as a string enum  - `INSIDE`: stroke drawn inside the shape boundary - `OUTSIDE`: stroke drawn outside the shape boundary - `CENTER`: stroke drawn centered along the shape boundary
    #[serde(rename = "strokeAlign", skip_serializing_if = "Option::is_none")]
    pub stroke_align: Option<StrokeAlign>,
    /// A string enum with value of \"MITER\", \"BEVEL\", or \"ROUND\", describing how corners in vector paths are rendered.
    #[serde(rename = "strokeJoin", skip_serializing_if = "Option::is_none")]
    pub stroke_join: Option<StrokeJoin>,
    /// An array of floating point numbers describing the pattern of dash length and gap lengths that the vector stroke will use when drawn.  For example a value of [1, 2] indicates that the stroke will be drawn with a dash of length 1 followed by a gap of length 2, repeated.
    #[serde(rename = "strokeDashes", skip_serializing_if = "Option::is_none")]
    pub stroke_dashes: Option<Vec<f64>>,
    /// Only specified if parameter `geometry=paths` is used. An array of paths representing the object fill.
    #[serde(rename = "fillGeometry", skip_serializing_if = "Option::is_none")]
    pub fill_geometry: Option<Vec<models::Path>>,
    /// Only specified if parameter `geometry=paths` is used. An array of paths representing the object stroke.
    #[serde(rename = "strokeGeometry", skip_serializing_if = "Option::is_none")]
    pub stroke_geometry: Option<Vec<models::Path>>,
    /// A string enum describing the end caps of vector paths.
    #[serde(rename = "strokeCap", skip_serializing_if = "Option::is_none")]
    pub stroke_cap: Option<StrokeCap>,
    /// Only valid if `strokeJoin` is \"MITER\". The corner angle, in degrees, below which `strokeJoin` will be set to \"BEVEL\" to avoid super sharp corners. By default this is 28.96 degrees.
    #[serde(rename = "strokeMiterAngle", skip_serializing_if = "Option::is_none")]
    pub stroke_miter_angle: Option<f64>,
}

impl HasGeometryTrait {
    pub fn new(fills: Vec<models::Paint>) -> HasGeometryTrait {
        HasGeometryTrait {
            fills,
            styles: None,
            strokes: None,
            stroke_weight: None,
            stroke_align: None,
            stroke_join: None,
            stroke_dashes: None,
            fill_geometry: None,
            stroke_geometry: None,
            stroke_cap: None,
            stroke_miter_angle: None,
        }
    }
}
/// Position of stroke relative to vector outline, as a string enum  - `INSIDE`: stroke drawn inside the shape boundary - `OUTSIDE`: stroke drawn outside the shape boundary - `CENTER`: stroke drawn centered along the shape boundary
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StrokeAlign {
    #[serde(rename = "INSIDE")]
    Inside,
    #[serde(rename = "OUTSIDE")]
    Outside,
    #[serde(rename = "CENTER")]
    Center,
}

impl Default for StrokeAlign {
    fn default() -> StrokeAlign {
        Self::Inside
    }
}
/// A string enum with value of \"MITER\", \"BEVEL\", or \"ROUND\", describing how corners in vector paths are rendered.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StrokeJoin {
    #[serde(rename = "MITER")]
    Miter,
    #[serde(rename = "BEVEL")]
    Bevel,
    #[serde(rename = "ROUND")]
    Round,
}

impl Default for StrokeJoin {
    fn default() -> StrokeJoin {
        Self::Miter
    }
}
/// A string enum describing the end caps of vector paths.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StrokeCap {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ROUND")]
    Round,
    #[serde(rename = "SQUARE")]
    Square,
    #[serde(rename = "LINE_ARROW")]
    LineArrow,
    #[serde(rename = "TRIANGLE_ARROW")]
    TriangleArrow,
    #[serde(rename = "DIAMOND_FILLED")]
    DiamondFilled,
    #[serde(rename = "CIRCLE_FILLED")]
    CircleFilled,
    #[serde(rename = "TRIANGLE_FILLED")]
    TriangleFilled,
    #[serde(rename = "WASHI_TAPE_1")]
    WashiTape1,
    #[serde(rename = "WASHI_TAPE_2")]
    WashiTape2,
    #[serde(rename = "WASHI_TAPE_3")]
    WashiTape3,
    #[serde(rename = "WASHI_TAPE_4")]
    WashiTape4,
    #[serde(rename = "WASHI_TAPE_5")]
    WashiTape5,
    #[serde(rename = "WASHI_TAPE_6")]
    WashiTape6,
}

impl Default for StrokeCap {
    fn default() -> StrokeCap {
        Self::None
    }
}

