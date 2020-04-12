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

/// EventInsights2016 : Insights for FIRST Stronghold qualification and elimination matches.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventInsights2016 {
    /// For the Low Bar - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "LowBar")]
    pub low_bar: Vec<f32>,
    /// For the Cheval De Frise - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "A_ChevalDeFrise")]
    pub a_cheval_de_frise: Vec<f32>,
    /// For the Portcullis - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "A_Portcullis")]
    pub a_portcullis: Vec<f32>,
    /// For the Ramparts - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "B_Ramparts")]
    pub b_ramparts: Vec<f32>,
    /// For the Moat - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "B_Moat")]
    pub b_moat: Vec<f32>,
    /// For the Sally Port - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "C_SallyPort")]
    pub c_sally_port: Vec<f32>,
    /// For the Drawbridge - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "C_Drawbridge")]
    pub c_drawbridge: Vec<f32>,
    /// For the Rough Terrain - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "D_RoughTerrain")]
    pub d_rough_terrain: Vec<f32>,
    /// For the Rock Wall - An array with three values, number of times damaged, number of opportunities to damage, and percentage.
    #[serde(rename = "D_RockWall")]
    pub d_rock_wall: Vec<f32>,
    /// Average number of high goals scored.
    #[serde(rename = "average_high_goals")]
    pub average_high_goals: f32,
    /// Average number of low goals scored.
    #[serde(rename = "average_low_goals")]
    pub average_low_goals: f32,
    /// An array with three values, number of times breached, number of opportunities to breach, and percentage.
    #[serde(rename = "breaches")]
    pub breaches: Vec<f32>,
    /// An array with three values, number of times scaled, number of opportunities to scale, and percentage.
    #[serde(rename = "scales")]
    pub scales: Vec<f32>,
    /// An array with three values, number of times challenged, number of opportunities to challenge, and percentage.
    #[serde(rename = "challenges")]
    pub challenges: Vec<f32>,
    /// An array with three values, number of times captured, number of opportunities to capture, and percentage.
    #[serde(rename = "captures")]
    pub captures: Vec<f32>,
    /// Average winning score.
    #[serde(rename = "average_win_score")]
    pub average_win_score: f32,
    /// Average margin of victory.
    #[serde(rename = "average_win_margin")]
    pub average_win_margin: f32,
    /// Average total score.
    #[serde(rename = "average_score")]
    pub average_score: f32,
    /// Average autonomous score.
    #[serde(rename = "average_auto_score")]
    pub average_auto_score: f32,
    /// Average crossing score.
    #[serde(rename = "average_crossing_score")]
    pub average_crossing_score: f32,
    /// Average boulder score.
    #[serde(rename = "average_boulder_score")]
    pub average_boulder_score: f32,
    /// Average tower score.
    #[serde(rename = "average_tower_score")]
    pub average_tower_score: f32,
    /// Average foul score.
    #[serde(rename = "average_foul_score")]
    pub average_foul_score: f32,
    /// An array with three values, high score, match key from the match with the high score, and the name of the match.
    #[serde(rename = "high_score")]
    pub high_score: Vec<String>,
}

impl EventInsights2016 {
    /// Insights for FIRST Stronghold qualification and elimination matches.
    pub fn new(
        low_bar: Vec<f32>,
        a_cheval_de_frise: Vec<f32>,
        a_portcullis: Vec<f32>,
        b_ramparts: Vec<f32>,
        b_moat: Vec<f32>,
        c_sally_port: Vec<f32>,
        c_drawbridge: Vec<f32>,
        d_rough_terrain: Vec<f32>,
        d_rock_wall: Vec<f32>,
        average_high_goals: f32,
        average_low_goals: f32,
        breaches: Vec<f32>,
        scales: Vec<f32>,
        challenges: Vec<f32>,
        captures: Vec<f32>,
        average_win_score: f32,
        average_win_margin: f32,
        average_score: f32,
        average_auto_score: f32,
        average_crossing_score: f32,
        average_boulder_score: f32,
        average_tower_score: f32,
        average_foul_score: f32,
        high_score: Vec<String>,
    ) -> EventInsights2016 {
        EventInsights2016 {
            low_bar,
            a_cheval_de_frise,
            a_portcullis,
            b_ramparts,
            b_moat,
            c_sally_port,
            c_drawbridge,
            d_rough_terrain,
            d_rock_wall,
            average_high_goals,
            average_low_goals,
            breaches,
            scales,
            challenges,
            captures,
            average_win_score,
            average_win_margin,
            average_score,
            average_auto_score,
            average_crossing_score,
            average_boulder_score,
            average_tower_score,
            average_foul_score,
            high_score,
        }
    }
}