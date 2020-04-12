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
pub struct ZebraAlliances {
    /// Zebra MotionWorks data for teams on the red alliance
    #[serde(rename = "red", skip_serializing_if = "Option::is_none")]
    pub red: Option<Vec<crate::models::ZebraTeam>>,
    /// Zebra data for teams on the blue alliance
    #[serde(rename = "blue", skip_serializing_if = "Option::is_none")]
    pub blue: Option<Vec<crate::models::ZebraTeam>>,
}

impl ZebraAlliances {
    pub fn new() -> ZebraAlliances {
        ZebraAlliances {
            red: None,
            blue: None,
        }
    }
}
