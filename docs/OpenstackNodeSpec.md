# OpenstackNodeSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability_zone** | Option<**String**> | if not set, the default AZ from the Datacenter spec will be used | [optional]
**disk_size** | Option<**i64**> | if set, the rootDisk will be a volume. If not, the rootDisk will be on ephemeral storage and its size will be derived from the flavor | [optional]
**flavor** | **String** | instance flavor | 
**image** | **String** | image to use | 
**instance_ready_check_period** | Option<**String**> | Period of time to check for instance ready status, i.e. 10s/1m | [optional]
**instance_ready_check_timeout** | Option<**String**> | Max time to wait for the instance to be ready, i.e. 10s/1m | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Additional metadata to set | [optional]
**use_floating_ip** | Option<**bool**> | Defines whether floating ip should be used | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


