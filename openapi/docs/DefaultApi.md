# \DefaultApi

All URIs are relative to *https://api.enphaseenergy.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**consumption_lifetime**](DefaultApi.md#consumption_lifetime) | **GET** /systems/{system_id}/consumption_lifetime | 
[**consumption_stats**](DefaultApi.md#consumption_stats) | **GET** /systems/{system_id}/consumption_stats | 
[**energy_lifetime**](DefaultApi.md#energy_lifetime) | **GET** /systems/{system_id}/energy_lifetime | 
[**envoys**](DefaultApi.md#envoys) | **GET** /systems/{system_id}/envoys | 
[**inventory**](DefaultApi.md#inventory) | **GET** /systems/{system_id}/inventory | 
[**inverters_summary_by_envoy_or_site**](DefaultApi.md#inverters_summary_by_envoy_or_site) | **GET** /systems/inverters_summary_by_envoy_or_site | 
[**monthly_production**](DefaultApi.md#monthly_production) | **GET** /systems/{system_id}/monthly_production | 
[**production_meter_readings**](DefaultApi.md#production_meter_readings) | **GET** /systems/{system_id}/production_meter_readings | 
[**rgm_stats**](DefaultApi.md#rgm_stats) | **GET** /systems/{system_id}/rgm_stats | 
[**search_system_id**](DefaultApi.md#search_system_id) | **GET** /systems/search_system_id | 
[**stats**](DefaultApi.md#stats) | **GET** /systems/{system_id}/stats | 
[**summary**](DefaultApi.md#summary) | **GET** /systems/{system_id}/summary | 
[**systems**](DefaultApi.md#systems) | **GET** /systems | 



## consumption_lifetime

> crate::models::ConsumptionLifetimeResponse consumption_lifetime(user_id, system_id, start_date, end_date)


Returns a time series of energy consumption as measured by the consumption meter installed on the specified system. All measurements are in Watt-hours. If the system does not have a meter, returns `204` - No Content. If you don't have permission to view consumption data, the response code is `401`.  The time series includes one entry for each day from the `start_date` to the `end_date`. There are no gaps in the time series. If the response includes trailing zeroes, such as [909, 4970, 0, 0, 0], then no data has been reported for the last days in the series. You can check the system's status in the `meta` attribute of the response to determine when the system last reported and whether it has communication or metering problems.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |
**start_date** | Option<**String**> | The date on which to start the time series. Defaults to the system's operational date. |  |
**end_date** | Option<**String**> | The last date to include in the time series. Defaults to yesterday or the last day the system reported, whichever is earlier. |  |

### Return type

[**crate::models::ConsumptionLifetimeResponse**](ConsumptionLifetimeResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## consumption_stats

> crate::models::ConsumptionStatsResponse consumption_stats(user_id, system_id, start_at, end_at)


Returns consumption as measured by the consumption meter installed on the specified system. If the total duration requested is more than one month, returns one month of intervals. Intervals are 15 minutes in length and start at the top of the hour.  Requests for times that do not fall on the 15-minute marks are rounded down. For example, a request for 08:01, 08:08, 08:11, or 08:14 is treated as a request for 08:00. Intervals are listed by their end dates; therefore, the first interval returned is 15 minutes after the requested start date.  If the system doesn't have any consumption meters installed, the response includes an empty intervals array.  If you don't have permission to view consumption data, the response code is `401`.  Under some conditions, data for a given period may be temporarily unavailable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |
**start_at** | Option<**i64**> | Start of period to report on in Unix epoch time. If no start is specified, the assumed start is midnight today, in the timezone of the system. If the start is earlier than one year ago, the response includes an empty intervals list. If the start is earlier than the system's `operational_date`, the response data begins with the first interval of the `operational_date`. |  |
**end_at** | Option<**i64**> | End of reporting period in Unix epoch time. If no end is specified, defaults to the time of the request. If the end is later than the last reported interval the response data ends with the last reported interval. |  |

### Return type

[**crate::models::ConsumptionStatsResponse**](ConsumptionStatsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## energy_lifetime

> crate::models::EnergyLifetimeResponse energy_lifetime(user_id, system_id, start_date, end_date, production)


Returns a time series of energy produced on the system over its lifetime. All measurements are in Watt-hours.  The time series includes one entry for each day from the `start_date` to the `end_date`. There are no gaps in the time series. If the response includes trailing zeroes, such as `[909, 4970, 0, 0, 0]`, then no energy has been reported for the last days in the series. You can check the system's status in the `meta` attribute of the response to determine when the system last reported and whether it has communication or production problems.  If the system has a meter, the time series includes data as measured by the microinverters until the first full day after the meter has been installed, when it switches to using the data as measured by the meter. This is called the \"merged time series\". In addition, the response includes the attribute `meter_start_date`, to indicate where in the time series the meter measurements begin to be used. You can retrieve the complete time series from the meter and from the microinverters by adding the parameter `production=all` to the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |
**start_date** | Option<**String**> | The date on which to start the time series. Defaults to the system's operational date. |  |
**end_date** | Option<**String**> | The last date to include in the time series. Defaults to yesterday or the last day the system reported, whichever is earlier. |  |
**production** | Option<**String**> | When `all`, returns the merged time series plus the time series as reported by the microinverters and the meter on the system. Other values are ignored. |  |

### Return type

[**crate::models::EnergyLifetimeResponse**](EnergyLifetimeResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## envoys

> crate::models::EnvoysResponse envoys(user_id, system_id)


Returns a listing of all active Envoys currently deployed on the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |

### Return type

[**crate::models::EnvoysResponse**](EnvoysResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory

> crate::models::InventoryResponse inventory(user_id, system_id)


Returns a listing of active devices on the given system. A device is considered active if it has not been retired in Enlighten. \"Active\" does not imply that the device is currently reporting, producing, or measuring energy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |

### Return type

[**crate::models::InventoryResponse**](InventoryResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inverters_summary_by_envoy_or_site

> Vec<crate::models::InvertersSummaryByEnvoyOrSiteResponse> inverters_summary_by_envoy_or_site(user_id, site_id)


Returns the summary along with the energy produced on the system over its lifetime.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**site_id** | **i32** | The identifier of the system. | [required] |

### Return type

[**Vec<crate::models::InvertersSummaryByEnvoyOrSiteResponse>**](InvertersSummaryByEnvoyOrSiteResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monthly_production

> crate::models::MonthlyProductionResponse monthly_production(user_id, system_id, start_date)


This endpoint is deprecated and will be removed in a future release. Use `production_meter_readings` or `energy_lifetime` instead.  Returns the energy production of the system for the month starting on the given date. The start date must be at least one month ago. If a meter or meters are installed on the system, measurements come from the meter; otherwise, measurements come from the microinverters.  This endpoint can return a response of Data Temporarily Unavailable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |
**start_date** | **String** | Start date for reporting period. The reporting period ends on the previous day of the next month; for example, a `start_date` of 2011-07-20 returns data through 2011-06-19. When the start date is the first of a calendar month, the end end date is the last day of that month. | [required] |

### Return type

[**crate::models::MonthlyProductionResponse**](MonthlyProductionResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_meter_readings

> crate::models::ProductionMeterReadingsResponse production_meter_readings(user_id, system_id, end_at)


Returns the last known \"odometer\" reading of each revenue-grade production meter on the system as of the requested time.  This endpoint includes entries for every production meter on the requested system, regardless of whether the meter is currently in service or retired. `read_at` is the time at which the reading was taken, and is always less than or equal to the requested `end_at`. Commonly, the reading will be within 30 minutes of the requested `end_at`; however, larger deltas can occur and do not necessarily mean there is a problem with the meter or the system it is on. Systems that are configured to report infrequently can show large deltas on all meters, especially when `end_at` is close to the current time. Meters that have been retired from a system will show an `end_at` that doesn't change, and that eventually is far away from the current time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |
**end_at** | Option<**i64**> |  |  |

### Return type

[**crate::models::ProductionMeterReadingsResponse**](ProductionMeterReadingsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rgm_stats

> crate::models::RgmStatsResponse rgm_stats(user_id, system_id, start_at, end_at)


Returns performance statistics as measured by the revenue-grade meters installed on the specified system. If the total duration requested is more than one month, returns one month of intervals. Intervals are 15 minutes in length and start at the top of the hour.  Requests for times that do not fall on the 15-minute marks are rounded down. For example, a request for 08:01, 08:08, 08:11, or 08:14 is treated as a request for 08:00. Intervals are listed by their end dates; therefore, the first interval returned is 15 minutes after the requested start date.  If the system doesn't have any revenue-grade meters installed, the response includes an empty intervals array.  Under some conditions, data for a given period may be temporarily unavailable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |
**start_at** | Option<**i64**> | Start of period to report on in Unix epoch time. If no start is specified, the assumed start is midnight today, in the timezone of the system. If the start is earlier than one year ago, the response includes an empty intervals list. If the start is earlier than the system's `operational_date`, the response data begins with the first interval of the `operational_date`. |  |
**end_at** | Option<**i64**> | End of reporting period in Unix epoch time. If no end is specified, defaults to the time of the request. If the end is later than the last reported interval the response data ends with the last reported interval. |  |

### Return type

[**crate::models::RgmStatsResponse**](RgmStatsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_system_id

> crate::models::SearchSystemIdResponse search_system_id(user_id, serial_num)


Get system ID by envoy serial number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**serial_num** | **String** | Serial number of the envoy. | [required] |

### Return type

[**crate::models::SearchSystemIdResponse**](SearchSystemIdResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stats

> crate::models::StatsResponse stats(user_id, system_id, start_at, end_at)


Returns performance statistics for the specified system as reported by microinverters installed on the system. If the total duration requested is more than one day, returns one day of intervals. Intervals are 5 minutes in length and start at the top of the hour.  Requests for times that do not fall on the 5-minute marks are rounded down. For example, a request for 08:01, 08:02, 08:03, or 08:04 is treated as a request for 08:00. Intervals are listed by their end dates; therefore, the first interval returned is 5 minutes after the requested start date.  The response includes intervals that have been reported for the requested period. Gaps in reporting are not filled with 0-value intervals. The dark hours on a system are an example of such a gap, because the microinverters do not produce at night.  Sometimes a request cannot be processed because the requested dates are invalid for the the system in question. Examples include asking for stats starting at a time that is later than the system's last reported interval, or asking for stats before a system has started production. In cases such as these, the response code is `422` and the response body includes an error reason as well as the parameters used to process the request.  If the system doesn't have any microinverters installed, the response includes an empty intervals array. Under some conditions, data for a given period may be temporarily unavailable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |
**start_at** | Option<**i64**> | Start of reporting period in Unix epoch time. If no start is specified, defaults to midnight today, in the timezone of the system. If the start date is earlier than one year ago today, the response includes an empty intervals list. If the start is earlier than the system's `operational_date`, the response data begins with the `operational_date`. |  |
**end_at** | Option<**i64**> | End of reporting period in Unix epoch time. If no end is specified, the assumed end is now. If the end is later than the last reporting interval the response data ends with the last reported interval. |  |

### Return type

[**crate::models::StatsResponse**](StatsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## summary

> crate::models::SummaryResponse summary(user_id, system_id, summary_date)


Returns summary information for the specified system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**system_id** | **i32** |  | [required] |
**summary_date** | Option<**String**> | Start of reporting period. If no `summary_date` is provided, the start is the current day at midnight site-local time. Otherwise, the start is midnight site-local time of the requested day. If the requested date cannot be parsed or is in the future, the response includes an informative error message and `422` status. |  |

### Return type

[**crate::models::SummaryResponse**](SummaryResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## systems

> crate::models::SystemsResponse systems(user_id, next, limit, system_id, system_id2, system_name, system_name2, status, status2, reference, reference2, installer, installer2, connection_type, connection_type2)


Returns a list of systems for which the user can make API requests. There is a limit to the number of systems that can be returned at one time. If the first request does not return a full list, use the `next` attribute in the response body to request the next page of systems. By default, systems are returned in batches of 100. The maximum page size is 1000.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**next** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 100]
**system_id** | Option<**i32**> |  |  |
**system_id2** | Option<[**Vec<i32>**](i32.md)> |  |  |
**system_name** | Option<**String**> |  |  |
**system_name2** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<[**crate::models::Status**](.md)> |  |  |
**status2** | Option<[**Vec<crate::models::Status>**](crate::models::Status.md)> |  |  |
**reference** | Option<**String**> |  |  |
**reference2** | Option<[**Vec<String>**](String.md)> |  |  |
**installer** | Option<**String**> |  |  |
**installer2** | Option<[**Vec<String>**](String.md)> |  |  |
**connection_type** | Option<[**crate::models::ConnectionType**](.md)> |  |  |
**connection_type2** | Option<[**Vec<crate::models::ConnectionType>**](crate::models::ConnectionType.md)> |  |  |

### Return type

[**crate::models::SystemsResponse**](SystemsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

