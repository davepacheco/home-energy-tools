# SystemsResponseSystems

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | **i32** | The Enlighten ID of the system. | 
**system_name** | **String** | The name of the system. Even if the system owner has indicated their site is anonymous for public lists, the actual system name is returned here for identification purposes. | 
**system_public_name** | **String** | The display name of the system. Use this when displaying the system name on a public list or view. | 
**reference** | Option<**String**> | If the calling user belongs to a company and that company has provided its own identifiers for a system, that ID is included here. Otherwise, this attribute is not returned. | [optional]
**other_references** | Option<**Vec<String>**> | If any other companies have provided their own identifiers for a system, those identifiers are included here. Otherwise, this attribute is not returned. | [optional]
**country** | **String** | The two-letter code for the country where the system is located. See [ISO_3166-1_alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) for reference. | 
**state** | **String** | The two-letter code for the state where the system is located. See [ISO_3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) for reference. | 
**city** | **String** | The name of the city where the system is located. | 
**postal_code** | **String** | The postal code where the system is located. | 
**timezone** | **String** | The timezone of the system. | 
**connection_type** | [**crate::models::ConnectionType**](ConnectionType.md) |  | 
**status** | **String** | The current status of the system. You can find this and more in the `meta` property. | 
**meta** | [**crate::models::Meta**](Meta.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


