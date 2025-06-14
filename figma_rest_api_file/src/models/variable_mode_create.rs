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

/// VariableModeCreate : An object that contains details about creating a `VariableMode`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VariableModeCreate {
    /// The action to perform for the variable mode.
    #[serde(rename = "action")]
    pub action: Action,
    /// A temporary id for this variable mode.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of this variable mode.
    #[serde(rename = "name")]
    pub name: String,
    /// The variable collection that will contain the mode. You can use the temporary id of a variable collection.
    #[serde(rename = "variableCollectionId")]
    pub variable_collection_id: String,
}

impl VariableModeCreate {
    /// An object that contains details about creating a `VariableMode`.
    pub fn new(action: Action, name: String, variable_collection_id: String) -> VariableModeCreate {
        VariableModeCreate {
            action,
            id: None,
            name,
            variable_collection_id,
        }
    }
}
/// The action to perform for the variable mode.
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

