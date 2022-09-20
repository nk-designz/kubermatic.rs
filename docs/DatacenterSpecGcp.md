# DatacenterSpecGcp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | Option<**String**> | Region to use, for example \"europe-west3\", for a full list of regions see https://cloud.google.com/compute/docs/regions-zones/ | [optional]
**regional** | Option<**bool**> | Optional: Regional clusters spread their resources across multiple availability zones. Refer to the official documentation for more details on this: https://cloud.google.com/kubernetes-engine/docs/concepts/regional-clusters | [optional]
**zone_suffixes** | Option<**Vec<String>**> | List of enabled zones, for example [a, c]. See the link above for the available zones in your chosen region. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


