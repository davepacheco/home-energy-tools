# InvertersSummaryByEnvoyOrSiteResponseMicroInverters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**model** | **String** | Model number of this Microinverter. | 
**part_number** | **String** | The Enphase part number of this Microinverter. | 
**serial_number** | **String** | The serial number of this Microinverter. | 
**sku** | **String** |  | 
**status** | **String** | The current status of this Microinverter. * `normal` - The microinverter is operating normally. * `power` - There is a production issue. * `micro` - The microinverter is not reporting. * `retired` - The microinverter is retired. | 
**power_produced** | **i32** |  | 
**proc_load** | **String** |  | 
**param_table** | **String** |  | 
**envoy_serial_number** | **String** |  | 
**energy** | [**crate::models::InvertersSummaryByEnvoyOrSiteResponseEnergy**](InvertersSummaryByEnvoyOrSiteResponse_energy.md) |  | 
**grid_profile** | **String** |  | 
**last_report_date** | [**String**](string.md) | The last time this device submitted a report, by default expressed in Unix epoch time. If Enlighten has no record of a report from this Envoy, returns null. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


