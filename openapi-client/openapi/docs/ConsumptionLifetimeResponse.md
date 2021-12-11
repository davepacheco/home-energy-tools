# ConsumptionLifetimeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | [**String**](string.md) | When no `start_date` parameter is specified on the request, this is the `operational_date` of the system. May be null if system has never produced. When a `start_date` parameter is included in the request, it is included here in the response. | 
**system_id** | Option<**i32**> | The identifier of the system. | [optional]
**consumption** | **Vec<i32>** | An array of consumption measurements, one for each day since consumption metering began, or one for each day of the requested period. | 
**meta** | [**crate::models::Meta**](Meta.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


