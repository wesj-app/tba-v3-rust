# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | TBA event key with the format yyyy[EVENT_CODE], where yyyy is the year, and EVENT_CODE is the event code of the event. | 
**name** | **String** | Official name of event on record either provided by FIRST or organizers of offseason event. | 
**event_code** | **String** | Event short code, as provided by FIRST. | 
**event_type** | **i32** | Event Type, as defined here: https://github.com/the-blue-alliance/the-blue-alliance/blob/master/consts/event_type.py#L2 | 
**district** | Option<[**crate::models::DistrictList**](District_List.md)> |  | [optional]
**city** | Option<**String**> | City, town, village, etc. the event is located in. | [optional]
**state_prov** | Option<**String**> | State or Province the event is located in. | [optional]
**country** | Option<**String**> | Country the event is located in. | [optional]
**start_date** | [**String**](string.md) | Event start date in `yyyy-mm-dd` format. | 
**end_date** | [**String**](string.md) | Event end date in `yyyy-mm-dd` format. | 
**year** | **i32** | Year the event data is for. | 
**short_name** | Option<**String**> | Same as `name` but doesn't include event specifiers, such as 'Regional' or 'District'. May be null. | [optional]
**event_type_string** | **String** | Event Type, eg Regional, District, or Offseason. | 
**week** | Option<**i32**> | Week of the event relative to the first official season event, zero-indexed. Only valid for Regionals, Districts, and District Championships. Null otherwise. (Eg. A season with a week 0 'preseason' event does not count, and week 1 events will show 0 here. Seasons with a week 0.5 regional event will show week 0 for those event(s) and week 1 for week 1 events and so on.) | [optional]
**address** | Option<**String**> | Address of the event's venue, if available. | [optional]
**postal_code** | Option<**String**> | Postal code from the event address. | [optional]
**gmaps_place_id** | Option<**String**> | Google Maps Place ID for the event address. | [optional]
**gmaps_url** | Option<**String**> | Link to address location on Google Maps. | [optional]
**lat** | Option<**f64**> | Latitude for the event address. | [optional]
**lng** | Option<**f64**> | Longitude for the event address. | [optional]
**location_name** | Option<**String**> | Name of the location at the address for the event, eg. Blue Alliance High School. | [optional]
**timezone** | Option<**String**> | Timezone name. | [optional]
**website** | Option<**String**> | The event's website, if any. | [optional]
**first_event_id** | Option<**String**> | The FIRST internal Event ID, used to link to the event on the FRC webpage. | [optional]
**first_event_code** | Option<**String**> | Public facing event code used by FIRST (on frc-events.firstinspires.org, for example) | [optional]
**webcasts** | Option<[**Vec<crate::models::Webcast>**](Webcast.md)> |  | [optional]
**division_keys** | Option<**Vec<String>**> | An array of event keys for the divisions at this event. | [optional]
**parent_event_key** | Option<**String**> | The TBA Event key that represents the event's parent. Used to link back to the event from a division event. It is also the inverse relation of `divison_keys`. | [optional]
**playoff_type** | Option<**i32**> | Playoff Type, as defined here: https://github.com/the-blue-alliance/the-blue-alliance/blob/master/consts/playoff_type.py#L4, or null. | [optional]
**playoff_type_string** | Option<**String**> | String representation of the `playoff_type`, or null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


