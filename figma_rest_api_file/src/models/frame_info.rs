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

/// FrameInfo : Data on the frame a component resides in.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameInfo {
    /// The ID of the frame node within the file.
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// The name of the frame node.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The background color of the frame node.
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// The ID of the page containing the frame node.
    #[serde(rename = "pageId")]
    pub page_id: String,
    /// The name of the page containing the frame node.
    #[serde(rename = "pageName")]
    pub page_name: String,
    #[serde(rename = "containingStateGroup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub containing_state_group: Option<Option<Box<models::FrameInfoContainingStateGroup>>>,
    #[serde(rename = "containingComponentSet", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub containing_component_set: Option<Option<Box<models::FrameInfoContainingComponentSet>>>,
}

impl FrameInfo {
    /// Data on the frame a component resides in.
    pub fn new(page_id: String, page_name: String) -> FrameInfo {
        FrameInfo {
            node_id: None,
            name: None,
            background_color: None,
            page_id,
            page_name,
            containing_state_group: None,
            containing_component_set: None,
        }
    }
}

