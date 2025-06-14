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

/// Version : A version of a file
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    /// Unique identifier for version
    #[serde(rename = "id")]
    pub id: String,
    /// The UTC ISO 8601 time at which the version was created
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The label given to the version in the editor
    #[serde(rename = "label", deserialize_with = "Option::deserialize")]
    pub label: Option<String>,
    /// The description of the version as entered in the editor
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// The user that created the version
    #[serde(rename = "user")]
    pub user: Box<models::User>,
    /// A URL to a thumbnail image of the file version.
    #[serde(rename = "thumbnail_url", skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}

impl Version {
    /// A version of a file
    pub fn new(id: String, created_at: String, label: Option<String>, description: Option<String>, user: models::User) -> Version {
        Version {
            id,
            created_at,
            label,
            description,
            user: Box::new(user),
            thumbnail_url: None,
        }
    }
}

