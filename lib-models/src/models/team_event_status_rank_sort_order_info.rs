/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).    A `User-Agent` header may need to be set to prevent a 403 Unauthorized error.
 *
 * The version of the OpenAPI document: 3.8.0
 *
 * Generated by: https://openapi-generator.tech
 */
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamEventStatusRankSortOrderInfo {
    /// The number of digits of precision used for this value, eg `2` would correspond to a value of `101.11` while `0` would correspond to `101`.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>,
    /// The descriptive name of the value used to sort the ranking.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl TeamEventStatusRankSortOrderInfo {
    pub fn new() -> TeamEventStatusRankSortOrderInfo {
        TeamEventStatusRankSortOrderInfo {
            precision: None,
            name: None,
        }
    }
}
