# \DistrictApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_district_events**](DistrictApi.md#get_district_events) | **get** /district/{district_key}/events | 
[**get_district_events_keys**](DistrictApi.md#get_district_events_keys) | **get** /district/{district_key}/events/keys | 
[**get_district_events_simple**](DistrictApi.md#get_district_events_simple) | **get** /district/{district_key}/events/simple | 
[**get_district_rankings**](DistrictApi.md#get_district_rankings) | **get** /district/{district_key}/rankings | 
[**get_district_teams**](DistrictApi.md#get_district_teams) | **get** /district/{district_key}/teams | 
[**get_district_teams_keys**](DistrictApi.md#get_district_teams_keys) | **get** /district/{district_key}/teams/keys | 
[**get_district_teams_simple**](DistrictApi.md#get_district_teams_simple) | **get** /district/{district_key}/teams/simple | 
[**get_districts_by_year**](DistrictApi.md#get_districts_by_year) | **get** /districts/{year} | 
[**get_event_district_points**](DistrictApi.md#get_event_district_points) | **get** /event/{event_key}/district_points | 
[**get_team_districts**](DistrictApi.md#get_team_districts) | **get** /team/{team_key}/districts | 



## get_district_events

> Vec<crate::models::Event> get_district_events(district_key, if_modified_since)


Gets a list of events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_events_keys

> Vec<String> get_district_events_keys(district_key, if_modified_since)


Gets a list of event keys for events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_events_simple

> Vec<crate::models::EventSimple> get_district_events_simple(district_key, if_modified_since)


Gets a short-form list of events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::EventSimple>**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_rankings

> Vec<crate::models::DistrictRanking> get_district_rankings(district_key, if_modified_since)


Gets a list of team district rankings for the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::DistrictRanking>**](District_Ranking.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams

> Vec<crate::models::Team> get_district_teams(district_key, if_modified_since)


Gets a list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams_keys

> Vec<String> get_district_teams_keys(district_key, if_modified_since)


Gets a list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams_simple

> Vec<crate::models::TeamSimple> get_district_teams_simple(district_key, if_modified_since)


Gets a short-form list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_districts_by_year

> Vec<crate::models::DistrictList> get_districts_by_year(year, if_modified_since)


Gets a list of districts and their corresponding district key, for the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::DistrictList>**](District_List.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_district_points

> crate::models::EventDistrictPoints get_event_district_points(event_key, if_modified_since)


Gets a list of team rankings for the Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::EventDistrictPoints**](Event_District_Points.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_districts

> Vec<crate::models::DistrictList> get_team_districts(team_key, if_modified_since)


Gets an array of districts representing each year the team was in a district. Will return an empty array if the team was never in a district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::DistrictList>**](District_List.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

