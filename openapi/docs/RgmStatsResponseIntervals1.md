# RgmStatsResponseIntervals1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel** | **i32** | The reporting channel identifier. | 
**end_at** | **i64** | End of interval. The format is Unix epoch time unless you pass a `datetime_format` parameter as described [here](https://developer.enphase.com/docs#Datetimes). | 
**wh_del** | Option<**f32**> | Energy delivered during this interval, in Watt-hours. | 
**curr_w** | Option<**i32**> | Energy delivered during this interval, in Watts. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


