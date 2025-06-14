/*
 * Figma API
 *
 * This is the OpenAPI specification for the [Figma REST API](https://www.figma.com/developers/api).  Note: we are releasing the OpenAPI specification as a beta given the large surface area and complexity of the REST API. If you notice any inaccuracies with the specification, please [file an issue](https://github.com/figma/rest-api-spec/issues).
 *
 * The version of the OpenAPI document: 0.31.0
 * Contact: support@figma.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_file_styles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFileStylesError {
    Status400(models::InlineObject16),
    Status403(models::InlineObject19),
    Status404(models::InlineObject22),
    Status429(models::InlineObject23),
    Status500(models::InlineObject25),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_style`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStyleError {
    Status400(models::InlineObject16),
    Status403(models::InlineObject19),
    Status404(models::InlineObject22),
    Status429(models::InlineObject23),
    Status500(models::InlineObject25),
    UnknownValue(serde_json::Value),
}


/// Get a list of published styles within a file library.
pub async fn get_file_styles(configuration: &configuration::Configuration, file_key: &str) -> Result<models::InlineObject11, Error<GetFileStylesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_file_key = file_key;

    let uri_str = format!("{}/v1/files/{file_key}/styles", configuration.base_path, file_key=crate::apis::urlencode(p_file_key));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Figma-Token", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::InlineObject11`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::InlineObject11`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFileStylesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get metadata on a style by key.
pub async fn get_style(configuration: &configuration::Configuration, key: &str) -> Result<models::InlineObject12, Error<GetStyleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_key = key;

    let uri_str = format!("{}/v1/styles/{key}", configuration.base_path, key=crate::apis::urlencode(p_key));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Figma-Token", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::InlineObject12`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::InlineObject12`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetStyleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

