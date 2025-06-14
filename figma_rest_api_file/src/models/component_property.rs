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

/// ComponentProperty : A property of a component.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentProperty {
    /// Type of this component property.
    #[serde(rename = "type")]
    pub r#type: models::ComponentPropertyType,
    #[serde(rename = "value")]
    pub value: Box<models::ComponentPropertyValue>,
    /// Preferred values for this property. Only applicable if type is `INSTANCE_SWAP`.
    #[serde(rename = "preferredValues", skip_serializing_if = "Option::is_none")]
    pub preferred_values: Option<Vec<models::InstanceSwapPreferredValue>>,
    #[serde(rename = "boundVariables", skip_serializing_if = "Option::is_none")]
    pub bound_variables: Option<Box<models::ComponentPropertyBoundVariables>>,
}

impl ComponentProperty {
    /// A property of a component.
    pub fn new(r#type: models::ComponentPropertyType, value: models::ComponentPropertyValue) -> ComponentProperty {
        ComponentProperty {
            r#type,
            value: Box::new(value),
            preferred_values: None,
            bound_variables: None,
        }
    }
}

