# \TeamApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_district_rankings**](TeamApi.md#get_district_rankings) | **get** /district/{district_key}/rankings | 
[**get_district_teams**](TeamApi.md#get_district_teams) | **get** /district/{district_key}/teams | 
[**get_district_teams_keys**](TeamApi.md#get_district_teams_keys) | **get** /district/{district_key}/teams/keys | 
[**get_district_teams_simple**](TeamApi.md#get_district_teams_simple) | **get** /district/{district_key}/teams/simple | 
[**get_event_teams**](TeamApi.md#get_event_teams) | **get** /event/{event_key}/teams | 
[**get_event_teams_keys**](TeamApi.md#get_event_teams_keys) | **get** /event/{event_key}/teams/keys | 
[**get_event_teams_simple**](TeamApi.md#get_event_teams_simple) | **get** /event/{event_key}/teams/simple | 
[**get_event_teams_statuses**](TeamApi.md#get_event_teams_statuses) | **get** /event/{event_key}/teams/statuses | 
[**get_team**](TeamApi.md#get_team) | **get** /team/{team_key} | 
[**get_team_awards**](TeamApi.md#get_team_awards) | **get** /team/{team_key}/awards | 
[**get_team_awards_by_year**](TeamApi.md#get_team_awards_by_year) | **get** /team/{team_key}/awards/{year} | 
[**get_team_districts**](TeamApi.md#get_team_districts) | **get** /team/{team_key}/districts | 
[**get_team_event_awards**](TeamApi.md#get_team_event_awards) | **get** /team/{team_key}/event/{event_key}/awards | 
[**get_team_event_matches**](TeamApi.md#get_team_event_matches) | **get** /team/{team_key}/event/{event_key}/matches | 
[**get_team_event_matches_keys**](TeamApi.md#get_team_event_matches_keys) | **get** /team/{team_key}/event/{event_key}/matches/keys | 
[**get_team_event_matches_simple**](TeamApi.md#get_team_event_matches_simple) | **get** /team/{team_key}/event/{event_key}/matches/simple | 
[**get_team_event_status**](TeamApi.md#get_team_event_status) | **get** /team/{team_key}/event/{event_key}/status | 
[**get_team_events**](TeamApi.md#get_team_events) | **get** /team/{team_key}/events | 
[**get_team_events_by_year**](TeamApi.md#get_team_events_by_year) | **get** /team/{team_key}/events/{year} | 
[**get_team_events_by_year_keys**](TeamApi.md#get_team_events_by_year_keys) | **get** /team/{team_key}/events/{year}/keys | 
[**get_team_events_by_year_simple**](TeamApi.md#get_team_events_by_year_simple) | **get** /team/{team_key}/events/{year}/simple | 
[**get_team_events_keys**](TeamApi.md#get_team_events_keys) | **get** /team/{team_key}/events/keys | 
[**get_team_events_simple**](TeamApi.md#get_team_events_simple) | **get** /team/{team_key}/events/simple | 
[**get_team_events_statuses_by_year**](TeamApi.md#get_team_events_statuses_by_year) | **get** /team/{team_key}/events/{year}/statuses | 
[**get_team_matches_by_year**](TeamApi.md#get_team_matches_by_year) | **get** /team/{team_key}/matches/{year} | 
[**get_team_matches_by_year_keys**](TeamApi.md#get_team_matches_by_year_keys) | **get** /team/{team_key}/matches/{year}/keys | 
[**get_team_matches_by_year_simple**](TeamApi.md#get_team_matches_by_year_simple) | **get** /team/{team_key}/matches/{year}/simple | 
[**get_team_media_by_tag**](TeamApi.md#get_team_media_by_tag) | **get** /team/{team_key}/media/tag/{media_tag} | 
[**get_team_media_by_tag_year**](TeamApi.md#get_team_media_by_tag_year) | **get** /team/{team_key}/media/tag/{media_tag}/{year} | 
[**get_team_media_by_year**](TeamApi.md#get_team_media_by_year) | **get** /team/{team_key}/media/{year} | 
[**get_team_robots**](TeamApi.md#get_team_robots) | **get** /team/{team_key}/robots | 
[**get_team_simple**](TeamApi.md#get_team_simple) | **get** /team/{team_key}/simple | 
[**get_team_social_media**](TeamApi.md#get_team_social_media) | **get** /team/{team_key}/social_media | 
[**get_team_years_participated**](TeamApi.md#get_team_years_participated) | **get** /team/{team_key}/years_participated | 
[**get_teams**](TeamApi.md#get_teams) | **get** /teams/{page_num} | 
[**get_teams_by_year**](TeamApi.md#get_teams_by_year) | **get** /teams/{year}/{page_num} | 
[**get_teams_by_year_keys**](TeamApi.md#get_teams_by_year_keys) | **get** /teams/{year}/{page_num}/keys | 
[**get_teams_by_year_simple**](TeamApi.md#get_teams_by_year_simple) | **get** /teams/{year}/{page_num}/simple | 
[**get_teams_keys**](TeamApi.md#get_teams_keys) | **get** /teams/{page_num}/keys | 
[**get_teams_simple**](TeamApi.md#get_teams_simple) | **get** /teams/{page_num}/simple | 



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


## get_team

> crate::models::Team get_team(team_key, if_modified_since)


Gets a `Team` object for the team referenced by the given key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_awards

> Vec<crate::models::Award> get_team_awards(team_key, if_modified_since)


Gets a list of awards the given team has won.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Award>**](Award.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_awards_by_year

> Vec<crate::models::Award> get_team_awards_by_year(team_key, year, if_modified_since)


Gets a list of awards the given team has won in a given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Award>**](Award.md)

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


## get_team_matches_by_year

> Vec<crate::models::ModelMatch> get_team_matches_by_year(team_key, year, if_modified_since)


Gets a list of matches for the given team and year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::ModelMatch>**](Match.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_matches_by_year_keys

> Vec<String> get_team_matches_by_year_keys(team_key, year, if_modified_since)


Gets a list of match keys for matches for the given team and year.

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


## get_team_matches_by_year_simple

> Vec<crate::models::MatchSimple> get_team_matches_by_year_simple(team_key, year, if_modified_since)


Gets a short-form list of matches for the given team and year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::MatchSimple>**](Match_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_media_by_tag

> Vec<crate::models::Media> get_team_media_by_tag(team_key, media_tag, if_modified_since)


Gets a list of Media (videos / pictures) for the given team and tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**media_tag** | **String** | Media Tag which describes the Media. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Media>**](Media.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_media_by_tag_year

> Vec<crate::models::Media> get_team_media_by_tag_year(team_key, media_tag, year, if_modified_since)


Gets a list of Media (videos / pictures) for the given team, tag and year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**media_tag** | **String** | Media Tag which describes the Media. | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Media>**](Media.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_media_by_year

> Vec<crate::models::Media> get_team_media_by_year(team_key, year, if_modified_since)


Gets a list of Media (videos / pictures) for the given team and year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Media>**](Media.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_robots

> Vec<crate::models::TeamRobot> get_team_robots(team_key, if_modified_since)


Gets a list of year and robot name pairs for each year that a robot name was provided. Will return an empty array if the team has never named a robot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::TeamRobot>**](Team_Robot.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_simple

> crate::models::TeamSimple get_team_simple(team_key, if_modified_since)


Gets a `Team_Simple` object for the team referenced by the given key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::TeamSimple**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_social_media

> Vec<crate::models::Media> get_team_social_media(team_key, if_modified_since)


Gets a list of Media (social media) for the given team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<crate::models::Media>**](Media.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_years_participated

> Vec<i32> get_team_years_participated(team_key, if_modified_since)


Gets a list of years in which the team participated in at least one competition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

**Vec<i32>**

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

