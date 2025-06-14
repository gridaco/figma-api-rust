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

/// Action : An action that is performed when a trigger is activated.
/// An action that is performed when a trigger is activated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    ActionOneOf(Box<models::ActionOneOf>),
    OpenUrlAction(Box<models::OpenUrlAction>),
    UpdateMediaRuntimeAction(Box<models::UpdateMediaRuntimeAction>),
    SetVariableAction(Box<models::SetVariableAction>),
    SetVariableModeAction(Box<models::SetVariableModeAction>),
    ConditionalAction(Box<models::ConditionalAction>),
    NodeAction(Box<models::NodeAction>),
}

impl Default for Action {
    fn default() -> Self {
        Self::ActionOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BACK")]
    Back,
    #[serde(rename = "CLOSE")]
    Close,
    #[serde(rename = "URL")]
    Url,
    #[serde(rename = "UPDATE_MEDIA_RUNTIME")]
    UpdateMediaRuntime,
    #[serde(rename = "SET_VARIABLE")]
    SetVariable,
    #[serde(rename = "SET_VARIABLE_MODE")]
    SetVariableMode,
    #[serde(rename = "CONDITIONAL")]
    Conditional,
    #[serde(rename = "NODE")]
    Node,
}

impl Default for Type {
    fn default() -> Type {
        Self::Back
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaAction {
    #[serde(rename = "SKIP_TO")]
    SkipTo,
}

impl Default for MediaAction {
    fn default() -> MediaAction {
        Self::SkipTo
    }
}

