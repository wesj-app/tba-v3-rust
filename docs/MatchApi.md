# \MatchApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_event_match_timeseries**](MatchApi.md#get_event_match_timeseries) | **get** /event/{event_key}/matches/timeseries | 
[**get_event_matches**](MatchApi.md#get_event_matches) | **get** /event/{event_key}/matches | 
[**get_event_matches_keys**](MatchApi.md#get_event_matches_keys) | **get** /event/{event_key}/matches/keys | 
[**get_event_matches_simple**](MatchApi.md#get_event_matches_simple) | **get** /event/{event_key}/matches/simple | 
[**get_match**](MatchApi.md#get_match) | **get** /match/{match_key} | 
[**get_match_simple**](MatchApi.md#get_match_simple) | **get** /match/{match_key}/simple | 
[**get_match_timeseries**](MatchApi.md#get_match_timeseries) | **get** /match/{match_key}/timeseries | 
[**get_match_zebra**](MatchApi.md#get_match_zebra) | **get** /match/{match_key}/zebra_motionworks | 
[**get_team_event_matches**](MatchApi.md#get_team_event_matches) | **get** /team/{team_key}/event/{event_key}/matches | 
[**get_team_event_matches_keys**](MatchApi.md#get_team_event_matches_keys) | **get** /team/{team_key}/event/{event_key}/matches/keys | 
[**get_team_event_matches_simple**](MatchApi.md#get_team_event_matches_simple) | **get** /team/{team_key}/event/{event_key}/matches/simple | 
[**get_team_matches_by_year**](MatchApi.md#get_team_matches_by_year) | **get** /team/{team_key}/matches/{year} | 
[**get_team_matches_by_year_keys**](MatchApi.md#get_team_matches_by_year_keys) | **get** /team/{team_key}/matches/{year}/keys | 
[**get_team_matches_by_year_simple**](MatchApi.md#get_team_matches_by_year_simple) | **get** /team/{team_key}/matches/{year}/simple | 



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


## get_match

> crate::models::ModelMatch get_match(match_key, if_modified_since)


Gets a `Match` object for the given match key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_key** | **String** | TBA Match Key, eg `2016nytr_qm1` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::ModelMatch**](Match.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_match_simple

> crate::models::MatchSimple get_match_simple(match_key, if_modified_since)


Gets a short-form `Match` object for the given match key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_key** | **String** | TBA Match Key, eg `2016nytr_qm1` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::MatchSimple**](Match_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_match_timeseries

> Vec<serde_json::Value> get_match_timeseries(match_key, if_modified_since)


Gets an array of game-specific Match Timeseries objects for the given match key or an empty array if not available. *WARNING:* This is *not* official data, and is subject to a significant possibility of error, or missing data. Do not rely on this data for any purpose. In fact, pretend we made it up. *WARNING:* This endpoint and corresponding data models are under *active development* and may change at any time, including in breaking ways.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_key** | **String** | TBA Match Key, eg `2016nytr_qm1` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_match_zebra

> crate::models::Zebra get_match_zebra(match_key, if_modified_since)


Gets Zebra MotionWorks data for a Match for the given match key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_key** | **String** | TBA Match Key, eg `2016nytr_qm1` | [required] |
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::Zebra**](Zebra.md)

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

