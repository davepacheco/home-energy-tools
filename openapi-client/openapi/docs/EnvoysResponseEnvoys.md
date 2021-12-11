# EnvoysResponseEnvoys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**envoy_id** | **i32** | The Enlighten ID of the Envoy. | 
**last_report_at** | **i64** | The last time this Envoy submitted a report, by default expressed in Unix epoch time. When the `datetime_format` query parameter is `iso8601`, `last_report_at` is in ISO 8601 format. If Enlighten has no record of a report from this Envoy, returns null. | 
**name** | **String** | The human-friendly name of this Envoy. | 
**part_number** | **String** | The Enphase part number of this Envoy. | 
**serial_number** | **String** | The serial number of this Envoy. | 
**status** | **String** | The current status of this Envoy. * `normal` - The Envoy is operating normally. * `comm` - The Envoy is not communicating to Enlighten. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


