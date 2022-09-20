# AgentPoolOptionalSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_labels** | Option<**::std::collections::HashMap<String, String>**> | NodeLabels - The node labels to be persisted across all nodes in agent pool. | [optional]
**node_taints** | Option<**Vec<String>**> | NodeTaints - The taints added to new nodes during node pool create and scale. For example, key=value:NoSchedule. Placing custom taints on system pool is not supported(except 'CriticalAddonsOnly' taint or taint effect is 'PreferNoSchedule'). Please refer to https://aka.ms/aks/system-taints for detail | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


