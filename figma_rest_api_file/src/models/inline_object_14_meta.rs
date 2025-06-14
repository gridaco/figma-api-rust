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
pub struct InlineObject14Meta {
    /// A map of variable ids to variables
    #[serde(rename = "variables")]
    pub variables: std::collections::HashMap<String, models::PublishedVariable>,
    /// A map of variable collection ids to variable collections
    #[serde(rename = "variableCollections")]
    pub variable_collections: std::collections::HashMap<String, models::PublishedVariableCollection>,
}

impl InlineObject14Meta {
    pub fn new(variables: std::collections::HashMap<String, models::PublishedVariable>, variable_collections: std::collections::HashMap<String, models::PublishedVariableCollection>) -> InlineObject14Meta {
        InlineObject14Meta {
            variables,
            variable_collections,
        }
    }
}

