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

/// BlendMode : This type is a string enum with the following possible values  Normal blends: - `PASS_THROUGH` (only applicable to objects with children) - `NORMAL`  Darken: - `DARKEN` - `MULTIPLY` - `LINEAR_BURN` - `COLOR_BURN`  Lighten: - `LIGHTEN` - `SCREEN` - `LINEAR_DODGE` - `COLOR_DODGE`  Contrast: - `OVERLAY` - `SOFT_LIGHT` - `HARD_LIGHT`  Inversion: - `DIFFERENCE` - `EXCLUSION`  Component: - `HUE` - `SATURATION` - `COLOR` - `LUMINOSITY`
/// This type is a string enum with the following possible values  Normal blends: - `PASS_THROUGH` (only applicable to objects with children) - `NORMAL`  Darken: - `DARKEN` - `MULTIPLY` - `LINEAR_BURN` - `COLOR_BURN`  Lighten: - `LIGHTEN` - `SCREEN` - `LINEAR_DODGE` - `COLOR_DODGE`  Contrast: - `OVERLAY` - `SOFT_LIGHT` - `HARD_LIGHT`  Inversion: - `DIFFERENCE` - `EXCLUSION`  Component: - `HUE` - `SATURATION` - `COLOR` - `LUMINOSITY`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BlendMode {
    #[serde(rename = "PASS_THROUGH")]
    PassThrough,
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "DARKEN")]
    Darken,
    #[serde(rename = "MULTIPLY")]
    Multiply,
    #[serde(rename = "LINEAR_BURN")]
    LinearBurn,
    #[serde(rename = "COLOR_BURN")]
    ColorBurn,
    #[serde(rename = "LIGHTEN")]
    Lighten,
    #[serde(rename = "SCREEN")]
    Screen,
    #[serde(rename = "LINEAR_DODGE")]
    LinearDodge,
    #[serde(rename = "COLOR_DODGE")]
    ColorDodge,
    #[serde(rename = "OVERLAY")]
    Overlay,
    #[serde(rename = "SOFT_LIGHT")]
    SoftLight,
    #[serde(rename = "HARD_LIGHT")]
    HardLight,
    #[serde(rename = "DIFFERENCE")]
    Difference,
    #[serde(rename = "EXCLUSION")]
    Exclusion,
    #[serde(rename = "HUE")]
    Hue,
    #[serde(rename = "SATURATION")]
    Saturation,
    #[serde(rename = "COLOR")]
    Color,
    #[serde(rename = "LUMINOSITY")]
    Luminosity,

}

impl std::fmt::Display for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PassThrough => write!(f, "PASS_THROUGH"),
            Self::Normal => write!(f, "NORMAL"),
            Self::Darken => write!(f, "DARKEN"),
            Self::Multiply => write!(f, "MULTIPLY"),
            Self::LinearBurn => write!(f, "LINEAR_BURN"),
            Self::ColorBurn => write!(f, "COLOR_BURN"),
            Self::Lighten => write!(f, "LIGHTEN"),
            Self::Screen => write!(f, "SCREEN"),
            Self::LinearDodge => write!(f, "LINEAR_DODGE"),
            Self::ColorDodge => write!(f, "COLOR_DODGE"),
            Self::Overlay => write!(f, "OVERLAY"),
            Self::SoftLight => write!(f, "SOFT_LIGHT"),
            Self::HardLight => write!(f, "HARD_LIGHT"),
            Self::Difference => write!(f, "DIFFERENCE"),
            Self::Exclusion => write!(f, "EXCLUSION"),
            Self::Hue => write!(f, "HUE"),
            Self::Saturation => write!(f, "SATURATION"),
            Self::Color => write!(f, "COLOR"),
            Self::Luminosity => write!(f, "LUMINOSITY"),
        }
    }
}

impl Default for BlendMode {
    fn default() -> BlendMode {
        Self::PassThrough
    }
}

