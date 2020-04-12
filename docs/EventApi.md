# \EventApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_district_events**](EventApi.md#get_district_events) | **get** /district/{district_key}/events | 
[**get_district_events_keys**](EventApi.md#get_district_events_keys) | **get** /district/{district_key}/events/keys | 
[**get_district_events_simple**](EventApi.md#get_district_events_simple) | **get** /district/{district_key}/events/simple | 
[**get_event**](EventApi.md#get_event) | **get** /event/{event_key} | 
[**get_event_alliances**](EventApi.md#get_event_alliances) | **get** /event/{event_key}/alliances | 
[**get_event_awards**](EventApi.md#get_event_awards) | **get** /event/{event_key}/awards | 
[**get_event_district_points**](EventApi.md#get_event_district_points) | **get** /event/{event_key}/district_points | 
[**get_event_insights**](EventApi.md#get_event_insights) | **get** /event/{event_key}/insights | 
[**get_event_match_timeseries**](EventApi.md#get_event_match_timeseries) | **get** /event/{event_key}/matches/timeseries | 
[**get_event_matches**](EventApi.md#get_event_matches) | **get** /event/{event_key}/matches | 
[**get_event_matches_keys**](EventApi.md#get_event_matches_keys) | **get** /event/{event_key}/matches/keys | 
[**get_event_matches_simple**](EventApi.md#get_event_matches_simple) | **get** /event/{event_key}/matches/simple | 
[**get_event_op_rs**](EventApi.md#get_event_op_rs) | **get** /event/{event_key}/oprs | 
[**get_event_predictions**](EventApi.md#get_event_predictions) | **get** /event/{event_key}/predictions | 
[**get_event_rankings**](EventApi.md#get_event_rankings) | **get** /event/{event_key}/rankings | 
[**get_event_simple**](EventApi.md#get_event_simple) | **get** /event/{event_key}/simple | 
[**get_event_teams**](EventApi.md#get_event_teams) | **get** /event/{event_key}/teams | 
[**get_event_teams_keys**](EventApi.md#get_event_teams_keys) | **get** /event/{event_key}/teams/keys | 
[**get_event_teams_simple**](EventApi.md#get_event_teams_simple) | **get** /event/{event_key}/teams/simple | 
[**get_event_teams_statuses**](EventApi.md#get_event_teams_statuses) | **get** /event/{event_key}/teams/statuses | 
[**get_events_by_year**](EventApi.md#get_events_by_year) | **get** /events/{year} | 
[**get_events_by_year_keys**](EventApi.md#get_events_by_year_keys) | **get** /events/{year}/keys | 
[**get_events_by_year_simple**](EventApi.md#get_events_by_year_simple) | **get** /events/{year}/simple | 
[**get_team_event_awards**](EventApi.md#get_team_event_awards) | **get** /team/{team_key}/event/{event_key}/awards | 
[**get_team_event_matches**](EventApi.md#get_team_event_matches) | **get** /team/{team_key}/event/{event_key}/matches | 
[**get_team_event_matches_keys**](EventApi.md#get_team_event_matches_keys) | **get** /team/{team_key}/event/{event_key}/matches/keys | 
[**get_team_event_matches_simple**](EventApi.md#get_team_event_matches_simple) | **get** /team/{team_key}/event/{event_key}/matches/simple | 
[**get_team_event_status**](EventApi.md#get_team_event_status) | **get** /team/{team_key}/event/{event_key}/status | 
[**get_team_events**](EventApi.md#get_team_events) | **get** /team/{team_key}/events | 
[**get_team_events_by_year**](EventApi.md#get_team_events_by_year) | **get** /team/{team_key}/events/{year} | 
[**get_team_events_by_year_keys**](EventApi.md#get_team_events_by_year_keys) | **get** /team/{team_key}/events/{year}/keys | 
[**get_team_events_by_year_simple**](EventApi.md#get_team_events_by_year_simple) | **get** /team/{team_key}/events/{year}/simple | 
[**get_team_events_keys**](EventApi.md#get_team_events_keys) | **get** /team/{team_key}/events/keys | 
[**get_team_events_simple**](EventApi.md#get_team_events_simple) | **get** /team/{team_key}/events/simple | 
[**get_team_events_statuses_by_year**](EventApi.md#get_team_events_statuses_by_year) | **get** /team/{team_key}/events/{year}/statuses | 



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


## get_event

> crate::models::Event get_event(event_key, if_modified_since)


Gets an Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::Event**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_alliances

> Vec<crate::models::EliminationAlliance> get_event_alliances(event_key, if_modified_since)


Gets a list of Elimination Alliances for the given Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::EliminationAlliance>**](Elimination_Alliance.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_awards

> Vec<crate::models::Award> get_event_awards(event_key, if_modified_since)


Gets a list of awards from the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Award>**](Award.md)

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


## get_event_insights

> crate::models::EventInsights get_event_insights(event_key, if_modified_since)


Gets a set of Event-specific insights for the given Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::EventInsights**](Event_Insights.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_match_timeseries

> Vec<String> get_event_match_timeseries(event_key, if_modified_since)


Gets an array of Match Keys for the given event key that have timeseries data. Returns an empty array if no matches have timeseries data. *WARNING:* This is *not* official data, and is subject to a significant possibility of error, or missing data. Do not rely on this data for any purpose. In fact, pretend we made it up. *WARNING:* This endpoint and corresponding data models are under *active development* and may change at any time, including in breaking ways.

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


## get_event_matches

> Vec<crate::models::ModelMatch> get_event_matches(event_key, if_modified_since)


Gets a list of matches for the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::ModelMatch>**](Match.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_matches_keys

> Vec<String> get_event_matches_keys(event_key, if_modified_since)


Gets a list of match keys for the given event.

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


## get_event_matches_simple

> Vec<crate::models::MatchSimple> get_event_matches_simple(event_key, if_modified_since)


Gets a short-form list of matches for the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::MatchSimple>**](Match_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_op_rs

> crate::models::EventOpRs get_event_op_rs(event_key, if_modified_since)


Gets a set of Event OPRs (including OPR, DPR, and CCWM) for the given Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::EventOpRs**](Event_OPRs.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_predictions

> serde_json::Value get_event_predictions(event_key, if_modified_since)


Gets information on TBA-generated predictions for the given Event. Contains year-specific information. *WARNING* This endpoint is currently under development and may change at any time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_rankings

> crate::models::EventRanking get_event_rankings(event_key, if_modified_since)


Gets a list of team rankings for the Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::EventRanking**](Event_Ranking.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_simple

> crate::models::EventSimple get_event_simple(event_key, if_modified_since)


Gets a short-form Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::EventSimple**](Event_Simple.md)

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


## get_team_event_awards

> Vec<crate::models::Award> get_team_event_awards(team_key, event_key, if_modified_since)


Gets a list of awards the given team won at the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Award>**](Award.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_event_matches

> Vec<crate::models::ModelMatch> get_team_event_matches(team_key, event_key, if_modified_since)


Gets a list of matches for the given team and event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::ModelMatch>**](Match.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_event_matches_keys

> Vec<String> get_team_event_matches_keys(team_key, event_key, if_modified_since)


Gets a list of match keys for matches for the given team and event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
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


## get_team_event_matches_simple

> Vec<crate::models::ModelMatch> get_team_event_matches_simple(team_key, event_key, if_modified_since)


Gets a short-form list of matches for the given team and event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::ModelMatch>**](Match.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_event_status

> crate::models::TeamEventStatus get_team_event_status(team_key, event_key, if_modified_since)


Gets the competition rank and status of the team at the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::TeamEventStatus**](Team_Event_Status.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events

> Vec<crate::models::Event> get_team_events(team_key, if_modified_since)


Gets a list of all events this team has competed at.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_by_year

> Vec<crate::models::Event> get_team_events_by_year(team_key, year, if_modified_since)


Gets a list of events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
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


## get_team_events_by_year_keys

> Vec<String> get_team_events_by_year_keys(team_key, year, if_modified_since)


Gets a list of the event keys for events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
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


## get_team_events_by_year_simple

> Vec<crate::models::EventSimple> get_team_events_by_year_simple(team_key, year, if_modified_since)


Gets a short-form list of events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
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


## get_team_events_keys

> Vec<String> get_team_events_keys(team_key, if_modified_since)


Gets a list of the event keys for all events this team has competed at.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_simple

> Vec<crate::models::EventSimple> get_team_events_simple(team_key, if_modified_since)


Gets a short-form list of all events this team has competed at.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
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

