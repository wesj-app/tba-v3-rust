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

/// WltRecord : A Win-Loss-Tie record for a team, or an alliance.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WltRecord {
    /// Number of losses.
    #[serde(rename = "losses")]
    pub losses: i32,
    /// Number of wins.
    #[serde(rename = "wins")]
    pub wins: i32,
    /// Number of ties.
    #[serde(rename = "ties")]
    pub ties: i32,
}

impl WltRecord {
    /// A Win-Loss-Tie record for a team, or an alliance.
    pub fn new(losses: i32, wins: i32, ties: i32) -> WltRecord {
        WltRecord { losses, wins, ties }
    }
}
