# NodeSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**containerd_registry_mirrors** | Option<[**crate::models::ContainerRuntimeContainerd**](ContainerRuntimeContainerd.md)> |  | [optional]
**http_proxy** | Option<**String**> |  | [optional]
**insecure_registries** | Option<**Vec<String>**> | Optional: These image registries will be configured as insecure on the container runtime. | [optional]
**no_proxy** | Option<**String**> |  | [optional]
**pause_image** | Option<**String**> | Optional: Translates to --pod-infra-container-image on the kubelet. If not set, the kubelet will default it. | [optional]
**registry_mirrors** | Option<**Vec<String>**> | Optional: These image registries will be configured as registry mirrors on the container runtime. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


