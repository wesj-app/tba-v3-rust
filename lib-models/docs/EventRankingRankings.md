# EventRankingRankings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**matches_played** | **i32** | Number of matches played by this team. | 
**qual_average** | Option<**i32**> | The average match score during qualifications. Year specific. May be null if not relevant for a given year. | [optional]
**extra_stats** | Option<**Vec<f32>**> | Additional special data on the team's performance calculated by TBA. | [optional]
**sort_orders** | Option<**Vec<f32>**> | Additional year-specific information, may be null. See parent `sort_order_info` for details. | [optional]
**record** | [**crate::models::WltRecord**](WLT_Record.md) |  | 
**rank** | **i32** | The team's rank at the event as provided by FIRST. | 
**dq** | **i32** | Number of times disqualified. | 
**team_key** | **String** | The team with this rank. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


