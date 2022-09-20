# GkeNodeConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**disk_size_gb** | Option<**i64**> | DiskSizeGb: Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB. | [optional]
**disk_type** | Option<**String**> | DiskType: Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard' | [optional]
**image_type** | Option<**String**> | ImageType: The image type to use for this node. Note that for a given image type, the latest version of it will be used. | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> | Labels: The map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node. In case of conflict in label keys, the applied set may differ depending on the Kubernetes version it's best to assume the behavior is undefined and conflicts should be avoided. For more information, including usage and the valid values, see: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ | [optional]
**local_ssd_count** | Option<**i64**> | LocalSsdCount: The number of local SSD disks to be attached to the node. The limit for this value is dependent upon the maximum number of disks available on a machine per zone. See: https://cloud.google.com/compute/docs/disks/local-ssd for more information. | [optional]
**machine_type** | Option<**String**> | MachineType: The name of a Google Compute Engine machine type (https://cloud.google.com/compute/docs/machine-types) If unspecified, the default machine type is `e2-medium`. | [optional]
**preemptible** | Option<**bool**> | Preemptible: Whether the nodes are created as preemptible VM instances. See: https://cloud.google.com/compute/docs/instances/preemptible for more information about preemptible VM instances. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


