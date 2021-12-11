# RgmStatsResponseIntervals

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_at** | **i64** | End of interval. The format is Unix epoch time unless you pass a `datetime_format` parameter as described [here](https://developer.enphase.com/docs#Datetimes). | 
**wh_del** | **i32** | Energy delivered during this interval, in Watt-hours. | 
**devices_reporting** | **i32** | Number of revenue-grade meters that reported data for this interval at the time of the request. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


