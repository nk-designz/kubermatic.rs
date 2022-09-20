# ClusterSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admission_plugins** | Option<**Vec<String>**> | Additional Admission Controller plugins | [optional]
**audit_logging** | Option<[**crate::models::AuditLoggingSettings**](AuditLoggingSettings.md)> |  | [optional]
**cloud** | Option<[**crate::models::CloudSpec**](CloudSpec.md)> |  | [optional]
**cluster_network** | Option<[**crate::models::ClusterNetworkingConfig**](ClusterNetworkingConfig.md)> |  | [optional]
**cni_plugin** | Option<[**crate::models::CniPluginSettings**](CNIPluginSettings.md)> |  | [optional]
**container_runtime** | Option<**String**> | ContainerRuntime to use, i.e. Docker or containerd. By default containerd will be used. | [optional]
**enable_operating_system_manager** | Option<**bool**> | EnableOperatingSystemManager enables OSM which in-turn is responsible for creating and managing worker node configuration. | [optional]
**enable_user_ssh_key_agent** | Option<**bool**> | EnableUserSSHKeyAgent control whether the UserSSHKeyAgent will be deployed in the user cluster or not. If it was enabled, the agent will be deployed and used to sync the user ssh keys, that the user attach to the created cluster. If the agent was disabled, it won't be deployed in the user cluster, thus after the cluster creation any attached ssh keys won't be synced to the worker nodes. Once the agent is enabled/disabled it cannot be changed after the cluster is being created. | [optional]
**event_rate_limit_config** | Option<[**crate::models::EventRateLimitConfig**](EventRateLimitConfig.md)> |  | [optional]
**kubernetes_dashboard** | Option<[**crate::models::KubernetesDashboard**](KubernetesDashboard.md)> |  | [optional]
**machine_networks** | Option<[**Vec<crate::models::MachineNetworkingConfig>**](MachineNetworkingConfig.md)> | MachineNetworks optionally specifies the parameters for IPAM. | [optional]
**mla** | Option<[**crate::models::MlaSettings**](MLASettings.md)> |  | [optional]
**oidc** | Option<[**crate::models::OidcSettings**](OIDCSettings.md)> |  | [optional]
**opa_integration** | Option<[**crate::models::OpaIntegrationSettings**](OPAIntegrationSettings.md)> |  | [optional]
**pod_node_selector_admission_plugin_config** | Option<**::std::collections::HashMap<String, String>**> | PodNodeSelectorAdmissionPluginConfig provides the configuration for the PodNodeSelector. It's used by the backend to create a configuration file for this plugin. The key:value from the map is converted to the namespace:<node-selectors-labels> in the file. The format in a file: podNodeSelectorPluginConfig: clusterDefaultNodeSelector: <node-selectors-labels> namespace1: <node-selectors-labels> namespace2: <node-selectors-labels> | [optional]
**service_account** | Option<[**crate::models::ServiceAccountSettings**](ServiceAccountSettings.md)> |  | [optional]
**update_window** | Option<[**crate::models::UpdateWindow**](UpdateWindow.md)> |  | [optional]
**use_event_rate_limit_admission_plugin** | Option<**bool**> | If active the EventRateLimit admission plugin is configured at the apiserver | [optional]
**use_pod_node_selector_admission_plugin** | Option<**bool**> | If active the PodNodeSelector admission plugin is configured at the apiserver | [optional]
**use_pod_security_policy_admission_plugin** | Option<**bool**> | If active the PodSecurityPolicy admission plugin is configured at the apiserver | [optional]
**version** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


