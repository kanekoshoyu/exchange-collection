/*
 * Coinbase API
 *
 * The Coinbase v2 API
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountsAccountIdPutRequest {
    /// New account name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AccountsAccountIdPutRequest {
    pub fn new() -> AccountsAccountIdPutRequest {
        AccountsAccountIdPutRequest {
            name: None,
        }
    }
}
