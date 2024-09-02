/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`sapi_v1_blvt_redeem_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1BlvtRedeemPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_blvt_redeem_record_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1BlvtRedeemRecordGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_blvt_subscribe_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1BlvtSubscribePostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_blvt_subscribe_record_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1BlvtSubscribeRecordGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_blvt_token_info_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1BlvtTokenInfoGetError {
    Status400(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_blvt_user_limit_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1BlvtUserLimitGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// Weight(IP): 1
pub async fn sapi_v1_blvt_redeem_post(configuration: &configuration::Configuration, token_name: &str, amount: f64, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1BlvtRedeemPost200Response, Error<SapiV1BlvtRedeemPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/blvt/redeem", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("tokenName", &token_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("amount", &amount.to_string())]);
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1BlvtRedeemPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// - Only the data of the latest 90 days is available  Weight(IP): 1
pub async fn sapi_v1_blvt_redeem_record_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, token_name: Option<&str>, id: Option<i64>, start_time: Option<i64>, end_time: Option<i64>, limit: Option<i32>, recv_window: Option<i64>) -> Result<Vec<models::SapiV1BlvtRedeemRecordGet200ResponseInner>, Error<SapiV1BlvtRedeemRecordGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/blvt/redeem/record", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = token_name {
        local_var_req_builder = local_var_req_builder.query(&[("tokenName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time {
        local_var_req_builder = local_var_req_builder.query(&[("startTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("endTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1BlvtRedeemRecordGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Weight(IP): 1
pub async fn sapi_v1_blvt_subscribe_post(configuration: &configuration::Configuration, token_name: &str, cost: f64, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1BlvtSubscribePost200Response, Error<SapiV1BlvtSubscribePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/blvt/subscribe", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("tokenName", &token_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("cost", &cost.to_string())]);
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1BlvtSubscribePostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// - Only the data of the latest 90 days is available  Weight(IP): 1
pub async fn sapi_v1_blvt_subscribe_record_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, token_name: Option<&str>, id: Option<i64>, start_time: Option<i64>, end_time: Option<i64>, limit: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1BlvtSubscribeRecordGet200Response, Error<SapiV1BlvtSubscribeRecordGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/blvt/subscribe/record", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = token_name {
        local_var_req_builder = local_var_req_builder.query(&[("tokenName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time {
        local_var_req_builder = local_var_req_builder.query(&[("startTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("endTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1BlvtSubscribeRecordGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Weight(IP): 1
pub async fn sapi_v1_blvt_token_info_get(configuration: &configuration::Configuration, token_name: Option<&str>) -> Result<Vec<models::SapiV1BlvtTokenInfoGet200ResponseInner>, Error<SapiV1BlvtTokenInfoGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/blvt/tokenInfo", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = token_name {
        local_var_req_builder = local_var_req_builder.query(&[("tokenName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1BlvtTokenInfoGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Weight(IP): 1
pub async fn sapi_v1_blvt_user_limit_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, token_name: Option<&str>, recv_window: Option<i64>) -> Result<Vec<models::SapiV1BlvtUserLimitGet200ResponseInner>, Error<SapiV1BlvtUserLimitGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/blvt/userLimit", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = token_name {
        local_var_req_builder = local_var_req_builder.query(&[("tokenName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1BlvtUserLimitGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

