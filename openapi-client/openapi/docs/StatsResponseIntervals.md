# StatsResponseIntervals

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_at** | **i64** | End date for interval. The format is Unix epoch time unless you pass a `datetime_format` parameter as described [here](https://developer.enphase.com/docs#Datetimes). | 
**powr** | **i32** | Average power produced by microinverters during this interval, in Watts. | 
**enwh** | **i32** | Energy produced by microinverters during this interval, in Watt-hours. | 
**devices_reporting** | **i32** | Number of microinverters that reported data for this interval at the time of the request. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


