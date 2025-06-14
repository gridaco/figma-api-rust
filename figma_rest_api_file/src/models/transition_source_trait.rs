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
pub struct TransitionSourceTrait {
    /// Node ID of node to transition to in prototyping
    #[serde(rename = "transitionNodeID", skip_serializing_if = "Option::is_none")]
    pub transition_node_id: Option<String>,
    /// The duration of the prototyping transition on this node (in milliseconds). This will override the default transition duration on the prototype, for this node.
    #[serde(rename = "transitionDuration", skip_serializing_if = "Option::is_none")]
    pub transition_duration: Option<f64>,
    /// The easing curve used in the prototyping transition on this node.
    #[serde(rename = "transitionEasing", skip_serializing_if = "Option::is_none")]
    pub transition_easing: Option<models::EasingType>,
}

impl TransitionSourceTrait {
    pub fn new() -> TransitionSourceTrait {
        TransitionSourceTrait {
            transition_node_id: None,
            transition_duration: None,
            transition_easing: None,
        }
    }
}

