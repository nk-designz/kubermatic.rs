# GkeShieldedInstanceConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_integrity_monitoring** | Option<**bool**> | EnableIntegrityMonitoring: Defines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the boot integrity of the instance. The attestation is performed against the integrity policy baseline. This baseline is initially derived from the implicitly trusted boot image when the instance is created. | [optional]
**enable_secure_boot** | Option<**bool**> | EnableSecureBoot: Defines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


