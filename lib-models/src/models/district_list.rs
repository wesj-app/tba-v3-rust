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
pub struct DistrictList {
    /// The short identifier for the district.
    #[serde(rename = "abbreviation")]
    pub abbreviation: String,
    /// The long name for the district.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Key for this district, e.g. `2016ne`.
    #[serde(rename = "key")]
    pub key: String,
    /// Year this district participated.
    #[serde(rename = "year")]
    pub year: i32,
}

impl DistrictList {
    pub fn new(abbreviation: String, display_name: String, key: String, year: i32) -> DistrictList {
        DistrictList {
            abbreviation,
            display_name,
            key,
            year,
        }
    }
}