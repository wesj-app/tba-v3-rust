# Team

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | TBA team key with the format `frcXXXX` with `XXXX` representing the team number. | 
**team_number** | **i32** | Official team number issued by FIRST. | 
**nickname** | Option<**String**> | Team nickname provided by FIRST. | [optional]
**name** | **String** | Official long name registered with FIRST. | 
**school_name** | Option<**String**> | Name of team school or affilited group registered with FIRST. | [optional]
**city** | Option<**String**> | City of team derived from parsing the address registered with FIRST. | [optional]
**state_prov** | Option<**String**> | State of team derived from parsing the address registered with FIRST. | [optional]
**country** | Option<**String**> | Country of team derived from parsing the address registered with FIRST. | [optional]
**address** | Option<**String**> | Will be NULL, for future development. | [optional]
**postal_code** | Option<**String**> | Postal code from the team address. | [optional]
**gmaps_place_id** | Option<**String**> | Will be NULL, for future development. | [optional]
**gmaps_url** | Option<**String**> | Will be NULL, for future development. | [optional]
**lat** | Option<**f64**> | Will be NULL, for future development. | [optional]
**lng** | Option<**f64**> | Will be NULL, for future development. | [optional]
**location_name** | Option<**String**> | Will be NULL, for future development. | [optional]
**website** | Option<**String**> | Official website associated with the team. | [optional]
**rookie_year** | Option<**i32**> | First year the team officially competed. | [optional]
**motto** | Option<**String**> | Team's motto as provided by FIRST. This field is deprecated and will return null - will be removed at end-of-season in 2019. | [optional]
**home_championship** | Option<[**serde_json::Value**](.md)> | Location of the team's home championship each year as a key-value pair. The year (as a string) is the key, and the city is the value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


