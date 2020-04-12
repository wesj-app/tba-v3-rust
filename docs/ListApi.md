# \ListApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_district_events**](ListApi.md#get_district_events) | **get** /district/{district_key}/events | 
[**get_district_events_keys**](ListApi.md#get_district_events_keys) | **get** /district/{district_key}/events/keys | 
[**get_district_events_simple**](ListApi.md#get_district_events_simple) | **get** /district/{district_key}/events/simple | 
[**get_district_rankings**](ListApi.md#get_district_rankings) | **get** /district/{district_key}/rankings | 
[**get_district_teams**](ListApi.md#get_district_teams) | **get** /district/{district_key}/teams | 
[**get_district_teams_keys**](ListApi.md#get_district_teams_keys) | **get** /district/{district_key}/teams/keys | 
[**get_district_teams_simple**](ListApi.md#get_district_teams_simple) | **get** /district/{district_key}/teams/simple | 
[**get_event_teams**](ListApi.md#get_event_teams) | **get** /event/{event_key}/teams | 
[**get_event_teams_keys**](ListApi.md#get_event_teams_keys) | **get** /event/{event_key}/teams/keys | 
[**get_event_teams_simple**](ListApi.md#get_event_teams_simple) | **get** /event/{event_key}/teams/simple | 
[**get_event_teams_statuses**](ListApi.md#get_event_teams_statuses) | **get** /event/{event_key}/teams/statuses | 
[**get_events_by_year**](ListApi.md#get_events_by_year) | **get** /events/{year} | 
[**get_events_by_year_keys**](ListApi.md#get_events_by_year_keys) | **get** /events/{year}/keys | 
[**get_events_by_year_simple**](ListApi.md#get_events_by_year_simple) | **get** /events/{year}/simple | 
[**get_team_events_statuses_by_year**](ListApi.md#get_team_events_statuses_by_year) | **get** /team/{team_key}/events/{year}/statuses | 
[**get_teams**](ListApi.md#get_teams) | **get** /teams/{page_num} | 
[**get_teams_by_year**](ListApi.md#get_teams_by_year) | **get** /teams/{year}/{page_num} | 
[**get_teams_by_year_keys**](ListApi.md#get_teams_by_year_keys) | **get** /teams/{year}/{page_num}/keys | 
[**get_teams_by_year_simple**](ListApi.md#get_teams_by_year_simple) | **get** /teams/{year}/{page_num}/simple | 
[**get_teams_keys**](ListApi.md#get_teams_keys) | **get** /teams/{page_num}/keys | 
[**get_teams_simple**](ListApi.md#get_teams_simple) | **get** /teams/{page_num}/simple | 



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


## get_event_teams

> Vec<crate::models::Team> get_event_teams(event_key, if_modified_since)


Gets a list of `Team` objects that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_keys

> Vec<String> get_event_teams_keys(event_key, if_modified_since)


Gets a list of `Team` keys that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_simple

> Vec<crate::models::TeamSimple> get_event_teams_simple(event_key, if_modified_since)


Gets a short-form list of `Team` objects that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_statuses

> ::std::collections::HashMap<String, crate::models::TeamEventStatus> get_event_teams_statuses(event_key, if_modified_since)


Gets a key-value list of the event statuses for teams competing at the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**::std::collections::HashMap<String, crate::models::TeamEventStatus>**](Team_Event_Status.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year

> Vec<crate::models::Event> get_events_by_year(year, if_modified_since)


Gets a list of events in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year_keys

> Vec<String> get_events_by_year_keys(year, if_modified_since)


Gets a list of event keys in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year_simple

> Vec<crate::models::EventSimple> get_events_by_year_simple(year, if_modified_since)


Gets a short-form list of events in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::EventSimple>**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_statuses_by_year

> ::std::collections::HashMap<String, crate::models::TeamEventStatus> get_team_events_statuses_by_year(team_key, year, if_modified_since)


Gets a key-value list of the event statuses for events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**::std::collections::HashMap<String, crate::models::TeamEventStatus>**](Team_Event_Status.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams

> Vec<crate::models::Team> get_teams(page_num, if_modified_since)


Gets a list of `Team` objects, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_by_year

> Vec<crate::models::Team> get_teams_by_year(year, page_num, if_modified_since)


Gets a list of `Team` objects that competed in the given year, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_by_year_keys

> Vec<String> get_teams_by_year_keys(year, page_num, if_modified_since)


Gets a list Team Keys that competed in the given year, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_by_year_simple

> Vec<crate::models::TeamSimple> get_teams_by_year_simple(year, page_num, if_modified_since)


Gets a list of short form `Team_Simple` objects that competed in the given year, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_keys

> Vec<String> get_teams_keys(page_num, if_modified_since)


Gets a list of Team keys, paginated in groups of 500. (Note, each page will not have 500 teams, but will include the teams within that range of 500.)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_simple

> Vec<crate::models::TeamSimple> get_teams_simple(page_num, if_modified_since)


Gets a list of short form `Team_Simple` objects, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

