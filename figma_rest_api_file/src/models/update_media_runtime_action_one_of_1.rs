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

/// UpdateMediaRuntimeActionOneOf1 : An action that updates the runtime of a media node by skipping forward or backward.  The `destinationId` is the node ID of the media node to update. If `destinationId` is `null`, the action will  update the media node that contains the action.  The `mediaAction` is the action to perform on the media node.  The `amountToSkip` is the amount of time to skip in seconds.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMediaRuntimeActionOneOf1 {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "destinationId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<Option<String>>,
    #[serde(rename = "mediaAction")]
    pub media_action: MediaAction,
    #[serde(rename = "amountToSkip")]
    pub amount_to_skip: f64,
}

impl UpdateMediaRuntimeActionOneOf1 {
    /// An action that updates the runtime of a media node by skipping forward or backward.  The `destinationId` is the node ID of the media node to update. If `destinationId` is `null`, the action will  update the media node that contains the action.  The `mediaAction` is the action to perform on the media node.  The `amountToSkip` is the amount of time to skip in seconds.
    pub fn new(r#type: Type, media_action: MediaAction, amount_to_skip: f64) -> UpdateMediaRuntimeActionOneOf1 {
        UpdateMediaRuntimeActionOneOf1 {
            r#type,
            destination_id: None,
            media_action,
            amount_to_skip,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "UPDATE_MEDIA_RUNTIME")]
    UpdateMediaRuntime,
}

impl Default for Type {
    fn default() -> Type {
        Self::UpdateMediaRuntime
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaAction {
    #[serde(rename = "SKIP_FORWARD")]
    SkipForward,
    #[serde(rename = "SKIP_BACKWARD")]
    SkipBackward,
}

impl Default for MediaAction {
    fn default() -> MediaAction {
        Self::SkipForward
    }
}

