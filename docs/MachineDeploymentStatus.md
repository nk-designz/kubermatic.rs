# MachineDeploymentStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_replicas** | Option<**i32**> | Total number of available machines (ready for at least minReadySeconds) targeted by this deployment. +optional | [optional]
**observed_generation** | Option<**i64**> | The generation observed by the deployment controller. +optional | [optional]
**ready_replicas** | Option<**i32**> | Total number of ready machines targeted by this deployment. +optional | [optional]
**replicas** | Option<**i32**> | Total number of non-terminated machines targeted by this deployment (their labels match the selector). +optional | [optional]
**unavailable_replicas** | Option<**i32**> | Total number of unavailable machines targeted by this deployment. This is the total number of machines that are still required for the deployment to have 100% available capacity. They may either be machines that are running but not yet available or machines that still have not been created. +optional | [optional]
**updated_replicas** | Option<**i32**> | Total number of non-terminated machines targeted by this deployment that have the desired template spec. +optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


