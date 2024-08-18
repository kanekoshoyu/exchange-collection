/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1ManagedSubaccountInfoGet200Response {
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "managerSubUserInfoVoList")]
    pub manager_sub_user_info_vo_list: Vec<models::SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner>,
}

impl SapiV1ManagedSubaccountInfoGet200Response {
    pub fn new(total: i32, manager_sub_user_info_vo_list: Vec<models::SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner>) -> SapiV1ManagedSubaccountInfoGet200Response {
        SapiV1ManagedSubaccountInfoGet200Response {
            total,
            manager_sub_user_info_vo_list,
        }
    }
}
