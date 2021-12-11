# MonthlyProductionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | **i32** | Enlighten ID for this system. | 
**start_date** | [**String**](string.md) | First day included in the reporting period. The format is `YYYY-mm-dd` unless you pass a `datetime_format` parameter as described [here](https://developer.enphase.com/docs#Datetimes). | 
**end_date** | [**String**](string.md) | Last day included in the reporting period. | 
**production_wh** | **i32** | Total production for the requested period in Watt-hours. | 
**meter_readings** | [**Vec<crate::models::MonthlyProductionResponseMeterReadings>**](MonthlyProductionResponse_meter_readings.md) | If the system has any revenue-grade meters installed, the meter readings at the beginning and end of the reporting period are included here. Otherwise, the array is empty. | 
**meta** | [**crate::models::Meta**](Meta.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


