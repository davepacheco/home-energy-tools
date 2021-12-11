# RgmStatsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | **i32** | Enlighten ID for this system. | 
**total_devices** | **i32** | Number of active revenue-grade meters for this system. | 
**meta** | [**crate::models::Meta**](Meta.md) |  | 
**intervals** | [**Vec<crate::models::RgmStatsResponseIntervals>**](RgmStatsResponse_intervals.md) | A list of intervals between the requested start and end times. | 
**meter_intervals** | [**Vec<crate::models::RgmStatsResponseMeterIntervals>**](RgmStatsResponse_meter_intervals.md) | A list of intervals of a meter between the requested start and end times. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


