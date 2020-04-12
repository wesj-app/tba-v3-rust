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
pub struct EventDistrictPointsPoints {
    /// Total points awarded at this event.
    #[serde(rename = "total")]
    pub total: i32,
    /// Points awarded for alliance selection
    #[serde(rename = "alliance_points")]
    pub alliance_points: i32,
    /// Points awarded for elimination match performance.
    #[serde(rename = "elim_points")]
    pub elim_points: i32,
    /// Points awarded for event awards.
    #[serde(rename = "award_points")]
    pub award_points: i32,
    /// Points awarded for qualification match performance.
    #[serde(rename = "qual_points")]
    pub qual_points: i32,
}

impl EventDistrictPointsPoints {
    pub fn new(
        total: i32,
        alliance_points: i32,
        elim_points: i32,
        award_points: i32,
        qual_points: i32,
    ) -> EventDistrictPointsPoints {
        EventDistrictPointsPoints {
            total,
            alliance_points,
            elim_points,
            award_points,
            qual_points,
        }
    }
}