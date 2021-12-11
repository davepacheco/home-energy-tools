# EnergyLifetimeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | [**String**](string.md) | When no `start_date` parameter is specified on the request, this is the `operational_date` of the system. May be null if system has never produced. When a `start_date` parameter is included in the request, it is included here in the response. | 
**system_id** | **i32** | The identifier of the system. | 
**production** | **Vec<i32>** | An array of production measurements, one for each day since the system started producing, or one for each day of the requested period. If the system has never produced energy, the array may be empty. | 
**micro_production** | Option<**Vec<i32>**> |  | [optional]
**meter_production** | Option<**Vec<i32>**> |  | [optional]
**meter_start_date** | Option<[**String**](string.md)> | The first day in the time series when measurements are taken from a meter instead of from microinverters. This field is not present unless the system has a meter. | [optional]
**meta** | [**crate::models::Meta**](Meta.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


