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

/// MatchScoreBreakdown2020 : See the 2020 FMS API documentation for a description of each value. https://frcevents2.docs.apiary.io/#/reference/match-results/score-details

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchScoreBreakdown2020 {
    #[serde(rename = "blue", skip_serializing_if = "Option::is_none")]
    pub blue: Option<crate::models::MatchScoreBreakdown2020Alliance>,
    #[serde(rename = "red", skip_serializing_if = "Option::is_none")]
    pub red: Option<crate::models::MatchScoreBreakdown2020Alliance>,
}

impl MatchScoreBreakdown2020 {
    /// See the 2020 FMS API documentation for a description of each value. https://frcevents2.docs.apiary.io/#/reference/match-results/score-details
    pub fn new() -> MatchScoreBreakdown2020 {
        MatchScoreBreakdown2020 {
            blue: None,
            red: None,
        }
    }
}
