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


/// struct for typed errors of method [`sapi_v1_loan_vip_borrow_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipBorrowPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_collateral_account_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipCollateralAccountGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_collateral_data_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipCollateralDataGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_loanable_data_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipLoanableDataGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_ongoing_orders_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipOngoingOrdersGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_renew_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipRenewPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_repay_history_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipRepayHistoryGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_repay_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipRepayPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_request_data_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipRequestDataGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_loan_vip_request_interest_rate_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1LoanVipRequestInterestRateGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// VIP loan is available for VIP users only.  Weight(UID): 6000
pub async fn sapi_v1_loan_vip_borrow_post(configuration: &configuration::Configuration, loan_account_id: i64, loan_amount: f32, collateral_account_id: &str, collateral_coin: &str, is_flexible_rate: &str, timestamp: i64, signature: &str, loan_coin: Option<&str>, loan_term: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipBorrowPost200Response, Error<SapiV1LoanVipBorrowPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/borrow", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("loanAccountId", &loan_account_id.to_string())]);
    if let Some(ref local_var_str) = loan_coin {
        local_var_req_builder = local_var_req_builder.query(&[("loanCoin", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("loanAmount", &loan_amount.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("collateralAccountId", &collateral_account_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("collateralCoin", &collateral_coin.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("isFlexibleRate", &is_flexible_rate.to_string())]);
    if let Some(ref local_var_str) = loan_term {
        local_var_req_builder = local_var_req_builder.query(&[("loanTerm", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipBorrowPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// VIP loan is available for VIP users only.  Weight(IP): 6000
pub async fn sapi_v1_loan_vip_collateral_account_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, order_id: Option<i64>, collateral_account_id: Option<i64>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipCollateralAccountGet200Response, Error<SapiV1LoanVipCollateralAccountGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/collateral/account", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = order_id {
        local_var_req_builder = local_var_req_builder.query(&[("orderId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = collateral_account_id {
        local_var_req_builder = local_var_req_builder.query(&[("collateralAccountId", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipCollateralAccountGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get collateral asset data.  Weight(IP): 400
pub async fn sapi_v1_loan_vip_collateral_data_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, collateral_coin: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipCollateralDataGet200Response, Error<SapiV1LoanVipCollateralDataGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/collateral/data", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = collateral_coin {
        local_var_req_builder = local_var_req_builder.query(&[("collateralCoin", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipCollateralDataGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get interest rate and borrow limit of loanable assets. The borrow limit is shown in USD value.  Weight(IP): 400
pub async fn sapi_v1_loan_vip_loanable_data_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, loan_coin: Option<&str>, vip_level: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipLoanableDataGet200Response, Error<SapiV1LoanVipLoanableDataGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/loanable/data", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = loan_coin {
        local_var_req_builder = local_var_req_builder.query(&[("loanCoin", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = vip_level {
        local_var_req_builder = local_var_req_builder.query(&[("vipLevel", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipLoanableDataGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// VIP loan is available for VIP users only.  Weight(IP): 400
pub async fn sapi_v1_loan_vip_ongoing_orders_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, order_id: Option<i64>, collateral_account_id: Option<i64>, loan_coin: Option<&str>, collateral_coin: Option<&str>, current: Option<i32>, limit: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipOngoingOrdersGet200Response, Error<SapiV1LoanVipOngoingOrdersGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/ongoing/orders", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = order_id {
        local_var_req_builder = local_var_req_builder.query(&[("orderId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = collateral_account_id {
        local_var_req_builder = local_var_req_builder.query(&[("collateralAccountId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = loan_coin {
        local_var_req_builder = local_var_req_builder.query(&[("loanCoin", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = collateral_coin {
        local_var_req_builder = local_var_req_builder.query(&[("collateralCoin", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = current {
        local_var_req_builder = local_var_req_builder.query(&[("current", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipOngoingOrdersGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// VIP loan is available for VIP users only.  Weight(UID): 6000
pub async fn sapi_v1_loan_vip_renew_post(configuration: &configuration::Configuration, timestamp: i64, signature: &str, order_id: Option<i64>, loan_term: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipRenewPost200Response, Error<SapiV1LoanVipRenewPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/renew", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = order_id {
        local_var_req_builder = local_var_req_builder.query(&[("orderId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = loan_term {
        local_var_req_builder = local_var_req_builder.query(&[("loanTerm", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipRenewPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// VIP loan is available for VIP users only.  Weight(IP): 400
pub async fn sapi_v1_loan_vip_repay_history_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, order_id: Option<i64>, loan_coin: Option<&str>, start_time: Option<i64>, end_time: Option<i64>, current: Option<i32>, limit: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipRepayHistoryGet200Response, Error<SapiV1LoanVipRepayHistoryGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/repay/history", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = order_id {
        local_var_req_builder = local_var_req_builder.query(&[("orderId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = loan_coin {
        local_var_req_builder = local_var_req_builder.query(&[("loanCoin", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time {
        local_var_req_builder = local_var_req_builder.query(&[("startTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("endTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = current {
        local_var_req_builder = local_var_req_builder.query(&[("current", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipRepayHistoryGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// VIP loan is available for VIP users only.  Weight(UID): 6000
pub async fn sapi_v1_loan_vip_repay_post(configuration: &configuration::Configuration, amount: f64, timestamp: i64, signature: &str, order_id: Option<i64>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipRepayPost200Response, Error<SapiV1LoanVipRepayPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/repay", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = order_id {
        local_var_req_builder = local_var_req_builder.query(&[("orderId", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<SapiV1LoanVipRepayPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get Application Status  Weight(UID): 400
pub async fn sapi_v1_loan_vip_request_data_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, current: Option<i32>, limit: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipRequestDataGet200Response, Error<SapiV1LoanVipRequestDataGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/request/data", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = current {
        local_var_req_builder = local_var_req_builder.query(&[("current", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipRequestDataGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get borrow interest rate.  Weight(UID): 400
pub async fn sapi_v1_loan_vip_request_interest_rate_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, loan_coin: Option<&str>, recv_window: Option<i64>) -> Result<Vec<models::SapiV1LoanVipRequestInterestRateGet200ResponseInner>, Error<SapiV1LoanVipRequestInterestRateGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/loan/vip/request/interestRate", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = loan_coin {
        local_var_req_builder = local_var_req_builder.query(&[("loanCoin", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SapiV1LoanVipRequestInterestRateGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

