# InventoryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | **i32** | Enlighten ID for this system. | 
**envoys** | Option<[**Vec<crate::models::InventoryResponseEnvoys>**](InventoryResponse_envoys.md)> | A list of Envoys on this system, including serial number. | [optional]
**inverters** | [**Vec<crate::models::InventoryResponseEnvoys>**](InventoryResponse_envoys.md) | A list of inverters on this system, including serial and model numbers. | 
**meters** | [**Vec<crate::models::InventoryResponseMeters>**](InventoryResponse_meters.md) | A list of meters on this system, including serial number, manufacturer, and model number. | 
**meta** | [**crate::models::Meta**](Meta.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


