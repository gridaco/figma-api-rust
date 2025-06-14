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

/// LocalVariableCollection : A grouping of related Variable objects each with the same modes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalVariableCollection {
    /// The unique identifier of this variable collection.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of this variable collection.
    #[serde(rename = "name")]
    pub name: String,
    /// The key of this variable collection.
    #[serde(rename = "key")]
    pub key: String,
    /// The modes of this variable collection.
    #[serde(rename = "modes")]
    pub modes: Vec<models::LocalVariableCollectionModesInner>,
    /// The id of the default mode.
    #[serde(rename = "defaultModeId")]
    pub default_mode_id: String,
    /// Whether this variable collection is remote.
    #[serde(rename = "remote")]
    pub remote: bool,
    /// Whether this variable collection is hidden when publishing the current file as a library.
    #[serde(rename = "hiddenFromPublishing")]
    pub hidden_from_publishing: bool,
    /// The ids of the variables in the collection. Note that the order of these variables is roughly the same as what is shown in Figma Design, however it does not account for groups. As a result, the order of these variables may not exactly reflect the exact ordering and grouping shown in the authoring UI.
    #[serde(rename = "variableIds")]
    pub variable_ids: Vec<String>,
}

impl LocalVariableCollection {
    /// A grouping of related Variable objects each with the same modes.
    pub fn new(id: String, name: String, key: String, modes: Vec<models::LocalVariableCollectionModesInner>, default_mode_id: String, remote: bool, hidden_from_publishing: bool, variable_ids: Vec<String>) -> LocalVariableCollection {
        LocalVariableCollection {
            id,
            name,
            key,
            modes,
            default_mode_id,
            remote,
            hidden_from_publishing,
            variable_ids,
        }
    }
}

