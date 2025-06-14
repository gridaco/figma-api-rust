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

/// VariableCollectionCreate : An object that contains details about creating a `VariableCollection`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VariableCollectionCreate {
    /// The action to perform for the variable collection.
    #[serde(rename = "action")]
    pub action: Action,
    /// A temporary id for this variable collection.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of this variable collection.
    #[serde(rename = "name")]
    pub name: String,
    /// The initial mode refers to the mode that is created by default. You can set a temporary id here, in order to reference this mode later in this request.
    #[serde(rename = "initialModeId", skip_serializing_if = "Option::is_none")]
    pub initial_mode_id: Option<String>,
    /// Whether this variable collection is hidden when publishing the current file as a library.
    #[serde(rename = "hiddenFromPublishing", skip_serializing_if = "Option::is_none")]
    pub hidden_from_publishing: Option<bool>,
}

impl VariableCollectionCreate {
    /// An object that contains details about creating a `VariableCollection`.
    pub fn new(action: Action, name: String) -> VariableCollectionCreate {
        VariableCollectionCreate {
            action,
            id: None,
            name,
            initial_mode_id: None,
            hidden_from_publishing: None,
        }
    }
}
/// The action to perform for the variable collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "CREATE")]
    Create,
}

impl Default for Action {
    fn default() -> Action {
        Self::Create
    }
}

