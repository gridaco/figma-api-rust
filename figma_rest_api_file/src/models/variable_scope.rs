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

/// VariableScope : Scopes allow a variable to be shown or hidden in the variable picker for various fields. This declutters the Figma UI if you have a large number of variables. Variable scopes are currently supported on `FLOAT`, `STRING`, and `COLOR` variables.  `ALL_SCOPES` is a special scope that means that the variable will be shown in the variable picker for all variable fields. If `ALL_SCOPES` is set, no additional scopes can be set.  `ALL_FILLS` is a special scope that means that the variable will be shown in the variable picker for all fill fields. If `ALL_FILLS` is set, no additional fill scopes can be set.  Valid scopes for `FLOAT` variables: - `ALL_SCOPES` - `TEXT_CONTENT` - `WIDTH_HEIGHT` - `GAP` - `STROKE_FLOAT` - `EFFECT_FLOAT` - `OPACITY` - `FONT_WEIGHT` - `FONT_SIZE` - `LINE_HEIGHT` - `LETTER_SPACING` - `PARAGRAPH_SPACING` - `PARAGRAPH_INDENT`  Valid scopes for `STRING` variables: - `ALL_SCOPES` - `TEXT_CONTENT` - `FONT_FAMILY` - `FONT_STYLE`  Valid scopes for `COLOR` variables: - `ALL_SCOPES` - `ALL_FILLS` - `FRAME_FILL` - `SHAPE_FILL` - `TEXT_FILL` - `STROKE_COLOR` - `EFFECT_COLOR`
/// Scopes allow a variable to be shown or hidden in the variable picker for various fields. This declutters the Figma UI if you have a large number of variables. Variable scopes are currently supported on `FLOAT`, `STRING`, and `COLOR` variables.  `ALL_SCOPES` is a special scope that means that the variable will be shown in the variable picker for all variable fields. If `ALL_SCOPES` is set, no additional scopes can be set.  `ALL_FILLS` is a special scope that means that the variable will be shown in the variable picker for all fill fields. If `ALL_FILLS` is set, no additional fill scopes can be set.  Valid scopes for `FLOAT` variables: - `ALL_SCOPES` - `TEXT_CONTENT` - `WIDTH_HEIGHT` - `GAP` - `STROKE_FLOAT` - `EFFECT_FLOAT` - `OPACITY` - `FONT_WEIGHT` - `FONT_SIZE` - `LINE_HEIGHT` - `LETTER_SPACING` - `PARAGRAPH_SPACING` - `PARAGRAPH_INDENT`  Valid scopes for `STRING` variables: - `ALL_SCOPES` - `TEXT_CONTENT` - `FONT_FAMILY` - `FONT_STYLE`  Valid scopes for `COLOR` variables: - `ALL_SCOPES` - `ALL_FILLS` - `FRAME_FILL` - `SHAPE_FILL` - `TEXT_FILL` - `STROKE_COLOR` - `EFFECT_COLOR`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VariableScope {
    #[serde(rename = "ALL_SCOPES")]
    AllScopes,
    #[serde(rename = "TEXT_CONTENT")]
    TextContent,
    #[serde(rename = "CORNER_RADIUS")]
    CornerRadius,
    #[serde(rename = "WIDTH_HEIGHT")]
    WidthHeight,
    #[serde(rename = "GAP")]
    Gap,
    #[serde(rename = "ALL_FILLS")]
    AllFills,
    #[serde(rename = "FRAME_FILL")]
    FrameFill,
    #[serde(rename = "SHAPE_FILL")]
    ShapeFill,
    #[serde(rename = "TEXT_FILL")]
    TextFill,
    #[serde(rename = "STROKE_COLOR")]
    StrokeColor,
    #[serde(rename = "STROKE_FLOAT")]
    StrokeFloat,
    #[serde(rename = "EFFECT_FLOAT")]
    EffectFloat,
    #[serde(rename = "EFFECT_COLOR")]
    EffectColor,
    #[serde(rename = "OPACITY")]
    Opacity,
    #[serde(rename = "FONT_FAMILY")]
    FontFamily,
    #[serde(rename = "FONT_STYLE")]
    FontStyle,
    #[serde(rename = "FONT_WEIGHT")]
    FontWeight,
    #[serde(rename = "FONT_SIZE")]
    FontSize,
    #[serde(rename = "LINE_HEIGHT")]
    LineHeight,
    #[serde(rename = "LETTER_SPACING")]
    LetterSpacing,
    #[serde(rename = "PARAGRAPH_SPACING")]
    ParagraphSpacing,
    #[serde(rename = "PARAGRAPH_INDENT")]
    ParagraphIndent,
    #[serde(rename = "FONT_VARIATIONS")]
    FontVariations,

}

impl std::fmt::Display for VariableScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AllScopes => write!(f, "ALL_SCOPES"),
            Self::TextContent => write!(f, "TEXT_CONTENT"),
            Self::CornerRadius => write!(f, "CORNER_RADIUS"),
            Self::WidthHeight => write!(f, "WIDTH_HEIGHT"),
            Self::Gap => write!(f, "GAP"),
            Self::AllFills => write!(f, "ALL_FILLS"),
            Self::FrameFill => write!(f, "FRAME_FILL"),
            Self::ShapeFill => write!(f, "SHAPE_FILL"),
            Self::TextFill => write!(f, "TEXT_FILL"),
            Self::StrokeColor => write!(f, "STROKE_COLOR"),
            Self::StrokeFloat => write!(f, "STROKE_FLOAT"),
            Self::EffectFloat => write!(f, "EFFECT_FLOAT"),
            Self::EffectColor => write!(f, "EFFECT_COLOR"),
            Self::Opacity => write!(f, "OPACITY"),
            Self::FontFamily => write!(f, "FONT_FAMILY"),
            Self::FontStyle => write!(f, "FONT_STYLE"),
            Self::FontWeight => write!(f, "FONT_WEIGHT"),
            Self::FontSize => write!(f, "FONT_SIZE"),
            Self::LineHeight => write!(f, "LINE_HEIGHT"),
            Self::LetterSpacing => write!(f, "LETTER_SPACING"),
            Self::ParagraphSpacing => write!(f, "PARAGRAPH_SPACING"),
            Self::ParagraphIndent => write!(f, "PARAGRAPH_INDENT"),
            Self::FontVariations => write!(f, "FONT_VARIATIONS"),
        }
    }
}

impl Default for VariableScope {
    fn default() -> VariableScope {
        Self::AllScopes
    }
}

