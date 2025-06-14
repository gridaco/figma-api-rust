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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Transition {
    SimpleTransition(Box<models::SimpleTransition>),
    DirectionalTransition(Box<models::DirectionalTransition>),
}

impl Default for Transition {
    fn default() -> Self {
        Self::SimpleTransition(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DISSOLVE")]
    Dissolve,
    #[serde(rename = "SMART_ANIMATE")]
    SmartAnimate,
    #[serde(rename = "SCROLL_ANIMATE")]
    ScrollAnimate,
    #[serde(rename = "MOVE_IN")]
    MoveIn,
    #[serde(rename = "MOVE_OUT")]
    MoveOut,
    #[serde(rename = "PUSH")]
    Push,
    #[serde(rename = "SLIDE_IN")]
    SlideIn,
    #[serde(rename = "SLIDE_OUT")]
    SlideOut,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dissolve
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "BOTTOM")]
    Bottom,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Left
    }
}

