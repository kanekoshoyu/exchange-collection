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
pub struct UsersUserIdGet200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::User>>,
}

impl UsersUserIdGet200Response {
    pub fn new() -> UsersUserIdGet200Response {
        UsersUserIdGet200Response {
            data: None,
        }
    }
}

