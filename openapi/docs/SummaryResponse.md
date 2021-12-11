# SummaryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_power** | **i32** | Current power production, in Watts. For historical requests, returns 0. | 
**energy_lifetime** | **i32** | Energy produced in the lifetime of the system, excluding the requested day, in Watt-hours. | 
**energy_today** | **i32** | Energy produced on the requested day, in Watt-hours. | 
**last_interval_end_at** | **i64** | The last known time that the system produced energy. When a system has not been communicating for a length of time, the `last_report_at` can be recent, whereas the `last_interval_end_at` may be further back. | 
**last_report_at** | **i64** | The last time an Envoy on this system reported. The format is Unix epoch time unless you pass a `datetime_format` parameter as described [here](https://developer.enphase.com/docs#Datetimes). | 
**modules** | **i32** | Number of active (not retired) modules. For historical requests, returns 0. | 
**operational_at** | **i64** | The time at which this system became operational. Corresponds to the system's interconnect time, if one is specified. Otherwise, it is the system's first interval end time. The format is Unix epoch time unless you pass a `datetime_format` parameter as described [here](https://developer.enphase.com/docs#Datetimes). | 
**size_w** | **i32** | The size of the system, in Watts. For historical requests, returns 0. | 
**source** | **String** | Indicates whether the production of this system is measured by its microinverters (`microinverters`) or by revenue-grade meters (`meter`) installed on the system. | 
**status** | [**crate::models::Status**](Status.md) |  | 
**summary_date** | [**String**](string.md) | Effective date of the response. For historical requests, returns the date requested. For current requests, returns the current date. The format is `YYYY-mm-dd` unless you pass a `datetime_format` parameter as described [here](https://developer.enphase.com/docs#Datetimes). | 
**system_id** | **i32** | The Enlighten ID of the system. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


