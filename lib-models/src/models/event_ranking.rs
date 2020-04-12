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
pub struct EventRanking {
    /// List of rankings at the event.
    #[serde(rename = "rankings")]
    pub rankings: Vec<crate::models::EventRankingRankings>,
    /// List of special TBA-generated values provided in the `extra_stats` array for each item.
    #[serde(rename = "extra_stats_info", skip_serializing_if = "Option::is_none")]
    pub extra_stats_info: Option<Vec<crate::models::EventRankingExtraStatsInfo>>,
    /// List of year-specific values provided in the `sort_orders` array for each team.
    #[serde(rename = "sort_order_info")]
    pub sort_order_info: Vec<crate::models::EventRankingSortOrderInfo>,
}

impl EventRanking {
    pub fn new(
        rankings: Vec<crate::models::EventRankingRankings>,
        sort_order_info: Vec<crate::models::EventRankingSortOrderInfo>,
    ) -> EventRanking {
        EventRanking {
            rankings,
            extra_stats_info: None,
            sort_order_info,
        }
    }
}
