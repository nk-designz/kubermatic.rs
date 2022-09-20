# \ProjectApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_ssh_key_to_cluster**](ProjectApi.md#assign_ssh_key_to_cluster) | **PUT** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/sshkeys/{key_id} | 
[**assign_ssh_key_to_cluster_v2**](ProjectApi.md#assign_ssh_key_to_cluster_v2) | **PUT** /api/v2/projects/{project_id}/clusters/{cluster_id}/sshkeys/{key_id} | 
[**bind_user_to_cluster_role**](ProjectApi.md#bind_user_to_cluster_role) | **POST** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/clusterroles/{role_id}/clusterbindings | 
[**bind_user_to_cluster_role_v2**](ProjectApi.md#bind_user_to_cluster_role_v2) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/clusterroles/{role_id}/clusterbindings | 
[**bind_user_to_role**](ProjectApi.md#bind_user_to_role) | **POST** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/roles/{namespace}/{role_id}/bindings | 
[**bind_user_to_role_v2**](ProjectApi.md#bind_user_to_role_v2) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/roles/{namespace}/{role_id}/bindings | 
[**create_cluster**](ProjectApi.md#create_cluster) | **POST** /api/v1/projects/{project_id}/dc/{dc}/clusters | Creates a cluster for the given project.
[**create_cluster_role**](ProjectApi.md#create_cluster_role) | **POST** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/clusterroles | 
[**create_cluster_template**](ProjectApi.md#create_cluster_template) | **POST** /api/v2/projects/{project_id}/clustertemplates | Creates a cluster templates for the given project.
[**create_cluster_template_instance**](ProjectApi.md#create_cluster_template_instance) | **POST** /api/v2/projects/{project_id}/clustertemplates/{template_id}/instances | Create cluster template instance.
[**create_cluster_v2**](ProjectApi.md#create_cluster_v2) | **POST** /api/v2/projects/{project_id}/clusters | Creates a cluster for the given project.
[**create_constraint**](ProjectApi.md#create_constraint) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/constraints | Creates a given constraint for the specified cluster.
[**create_external_cluster**](ProjectApi.md#create_external_cluster) | **POST** /api/v2/projects/{project_id}/kubernetes/clusters | Creates an external cluster for the given project.
[**create_external_cluster_machine_deployment**](ProjectApi.md#create_external_cluster_machine_deployment) | **POST** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments | Create an external cluster machine deployments.
[**create_gatekeeper_config**](ProjectApi.md#create_gatekeeper_config) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/gatekeeper/config | 
[**create_group_project_binding**](ProjectApi.md#create_group_project_binding) | **POST** /api/v2/projects/{project_id}/groupbindings | Create project group binding.
[**create_machine_deployment**](ProjectApi.md#create_machine_deployment) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments | 
[**create_node_deployment**](ProjectApi.md#create_node_deployment) | **POST** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodedeployments | 
[**create_project**](ProjectApi.md#create_project) | **POST** /api/v1/projects | Creates a brand new project.
[**create_role**](ProjectApi.md#create_role) | **POST** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/roles | 
[**create_ssh_key**](ProjectApi.md#create_ssh_key) | **POST** /api/v1/projects/{project_id}/sshkeys | Adds the given SSH key to the specified project.
[**delete_cluster**](ProjectApi.md#delete_cluster) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id} | 
[**delete_cluster_role**](ProjectApi.md#delete_cluster_role) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/clusterroles/{role_id} | 
[**delete_cluster_template**](ProjectApi.md#delete_cluster_template) | **DELETE** /api/v2/projects/{project_id}/clustertemplates/{template_id} | Delete cluster template.
[**delete_cluster_v2**](ProjectApi.md#delete_cluster_v2) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id} | 
[**delete_constraint**](ProjectApi.md#delete_constraint) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/constraints/{constraint_name} | Deletes a specified constraint for the given cluster.
[**delete_external_cluster**](ProjectApi.md#delete_external_cluster) | **DELETE** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id} | 
[**delete_external_cluster_machine_deployment**](ProjectApi.md#delete_external_cluster_machine_deployment) | **DELETE** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments/{machinedeployment_id} | Delete an external cluster machine deployment.
[**delete_gatekeeper_config**](ProjectApi.md#delete_gatekeeper_config) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/gatekeeper/config | Deletes the gatekeeper sync config for the specified cluster.
[**delete_group_project_binding**](ProjectApi.md#delete_group_project_binding) | **DELETE** /api/v2/projects/{project_id}/groupbindings/{binding_name} | Delete project group binding.
[**delete_machine_deployment**](ProjectApi.md#delete_machine_deployment) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments/{machinedeployment_id} | Deletes the given machine deployment that belongs to the cluster.
[**delete_machine_deployment_node**](ProjectApi.md#delete_machine_deployment_node) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments/nodes/{node_id} | Deletes the given node that belongs to the machine deployment.
[**delete_node_deployment**](ProjectApi.md#delete_node_deployment) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodedeployments/{nodedeployment_id} | Deletes the given node deployment that belongs to the cluster.
[**delete_node_for_cluster_legacy**](ProjectApi.md#delete_node_for_cluster_legacy) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodes/{node_id} | Deprecated: Deletes the given node that belongs to the cluster.
[**delete_project**](ProjectApi.md#delete_project) | **DELETE** /api/v1/projects/{project_id} | Deletes the project with the given ID.
[**delete_role**](ProjectApi.md#delete_role) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/roles/{namespace}/{role_id} | 
[**delete_ssh_key**](ProjectApi.md#delete_ssh_key) | **DELETE** /api/v1/projects/{project_id}/sshkeys/{key_id} | Removes the given SSH Key from the system.
[**detach_ssh_key_from_cluster**](ProjectApi.md#detach_ssh_key_from_cluster) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/sshkeys/{key_id} | 
[**detach_ssh_key_from_cluster_v2**](ProjectApi.md#detach_ssh_key_from_cluster_v2) | **DELETE** /api/projects/{project_id}/clusters/{cluster_id}/sshkeys/{key_id} | 
[**export_cluster_template**](ProjectApi.md#export_cluster_template) | **GET** /api/v2/projects/{project_id}/clustertemplates/{template_id}/export | Export cluster template to file.
[**get_alertmanager**](ProjectApi.md#get_alertmanager) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/alertmanager/config | Gets the alertmanager configuration for the specified cluster.
[**get_backup_destination_names**](ProjectApi.md#get_backup_destination_names) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/backupdestinations | 
[**get_cluster**](ProjectApi.md#get_cluster) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id} | 
[**get_cluster_events**](ProjectApi.md#get_cluster_events) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/events | Gets the events related to the specified cluster.
[**get_cluster_events_v2**](ProjectApi.md#get_cluster_events_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/events | Gets the events related to the specified cluster.
[**get_cluster_health**](ProjectApi.md#get_cluster_health) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/health | 
[**get_cluster_health_v2**](ProjectApi.md#get_cluster_health_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/health | 
[**get_cluster_kubeconfig**](ProjectApi.md#get_cluster_kubeconfig) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/kubeconfig | Gets the kubeconfig for the specified cluster.
[**get_cluster_kubeconfig_v2**](ProjectApi.md#get_cluster_kubeconfig_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/kubeconfig | Gets the kubeconfig for the specified cluster.
[**get_cluster_metrics**](ProjectApi.md#get_cluster_metrics) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/metrics | 
[**get_cluster_metrics_v2**](ProjectApi.md#get_cluster_metrics_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/metrics | 
[**get_cluster_oidc**](ProjectApi.md#get_cluster_oidc) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/oidc | Gets the OIDC params for the specified cluster with OIDC authentication.
[**get_cluster_role**](ProjectApi.md#get_cluster_role) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/roles/{role_id} | 
[**get_cluster_template**](ProjectApi.md#get_cluster_template) | **GET** /api/v2/projects/{project_id}/clustertemplates/{template_id} | Get cluster template.
[**get_cluster_upgrades**](ProjectApi.md#get_cluster_upgrades) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/upgrades | 
[**get_cluster_upgrades_v2**](ProjectApi.md#get_cluster_upgrades_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/upgrades | 
[**get_cluster_v2**](ProjectApi.md#get_cluster_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id} | 
[**get_constraint**](ProjectApi.md#get_constraint) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/constraints/{constraint_name} | Gets an specified constraint for the given cluster.
[**get_external_cluster**](ProjectApi.md#get_external_cluster) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id} | Gets an external cluster for the given project.
[**get_external_cluster_kubeconfig**](ProjectApi.md#get_external_cluster_kubeconfig) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/kubeconfig | Gets the kubeconfig for the specified external cluster.
[**get_external_cluster_machine_deployment**](ProjectApi.md#get_external_cluster_machine_deployment) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments/{machinedeployment_id} | Gets an external cluster machine deployments.
[**get_external_cluster_machine_deployment_upgrades**](ProjectApi.md#get_external_cluster_machine_deployment_upgrades) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments/{machinedeployment_id}/upgrades | Gets an external cluster machine deployments upgrade versions.
[**get_external_cluster_metrics**](ProjectApi.md#get_external_cluster_metrics) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/metrics | 
[**get_external_cluster_node**](ProjectApi.md#get_external_cluster_node) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/nodes/{node_id} | Gets an external cluster node.
[**get_external_cluster_upgrades**](ProjectApi.md#get_external_cluster_upgrades) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/upgrades | Gets an external cluster upgrades.
[**get_gatekeeper_config**](ProjectApi.md#get_gatekeeper_config) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/gatekeeper/config | Gets the gatekeeper sync config for the specified cluster.
[**get_group_project_binding**](ProjectApi.md#get_group_project_binding) | **GET** /api/v2/projects/{project_id}/groupbindings/{binding_name} | Get project group binding.
[**get_machine_deployment**](ProjectApi.md#get_machine_deployment) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments/{machinedeployment_id} | Gets a machine deployment that is assigned to the given cluster.
[**get_node_deployment**](ProjectApi.md#get_node_deployment) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodedeployments/{nodedeployment_id} | Gets a node deployment that is assigned to the given cluster.
[**get_oidc_cluster_kubeconfig**](ProjectApi.md#get_oidc_cluster_kubeconfig) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/oidckubeconfig | Gets the kubeconfig for the specified cluster with oidc authentication.
[**get_oidc_cluster_kubeconfig_v2**](ProjectApi.md#get_oidc_cluster_kubeconfig_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/oidckubeconfig | Gets the kubeconfig for the specified cluster with oidc authentication.
[**get_project**](ProjectApi.md#get_project) | **GET** /api/v1/projects/{project_id} | 
[**get_project_quota**](ProjectApi.md#get_project_quota) | **GET** /api/v2/projects/{project_id}/quota | Returns Resource Quota for a given project.
[**get_role**](ProjectApi.md#get_role) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/roles/{namespace}/{role_id} | 
[**import_cluster_template**](ProjectApi.md#import_cluster_template) | **POST** /api/v2/projects/{project_id}/clustertemplates/import | Import a cluster templates for the given project.
[**list_aks_clusters**](ProjectApi.md#list_aks_clusters) | **GET** /api/v2/projects/{project_id}/providers/aks/clusters | 
[**list_cluster_role**](ProjectApi.md#list_cluster_role) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/clusterroles | 
[**list_cluster_role_binding**](ProjectApi.md#list_cluster_role_binding) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/clusterbindings | 
[**list_cluster_role_binding_v2**](ProjectApi.md#list_cluster_role_binding_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/clusterbindings | 
[**list_cluster_role_names**](ProjectApi.md#list_cluster_role_names) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/clusterrolenames | 
[**list_cluster_role_names_v2**](ProjectApi.md#list_cluster_role_names_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/clusterrolenames | 
[**list_cluster_role_v2**](ProjectApi.md#list_cluster_role_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/clusterroles | 
[**list_cluster_templates**](ProjectApi.md#list_cluster_templates) | **GET** /api/v2/projects/{project_id}/clustertemplates | List cluster templates for the given project.
[**list_clusters**](ProjectApi.md#list_clusters) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters | Lists clusters for the specified project and data center.
[**list_clusters_for_project**](ProjectApi.md#list_clusters_for_project) | **GET** /api/v1/projects/{project_id}/clusters | Lists clusters for the specified project.
[**list_clusters_v2**](ProjectApi.md#list_clusters_v2) | **GET** /api/v2/projects/{project_id}/clusters | Lists clusters for the specified project. If query parameter `show_dm_count` is set to `true` then the endpoint will also return the number of machine deployments of each cluster.
[**list_cni_plugin_versions_for_cluster**](ProjectApi.md#list_cni_plugin_versions_for_cluster) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/cniversions | Lists CNI plugin versions for a given cluster.
[**list_constraints**](ProjectApi.md#list_constraints) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/constraints | Lists constraints for the specified cluster.
[**list_eks_clusters**](ProjectApi.md#list_eks_clusters) | **GET** /api/v2/projects/{project_id}/providers/eks/clusters | 
[**list_external_cluster_events**](ProjectApi.md#list_external_cluster_events) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/events | Gets an external cluster events.
[**list_external_cluster_machine_deployment_events**](ProjectApi.md#list_external_cluster_machine_deployment_events) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments/{machinedeployment_id}/nodes/events | List an external cluster machine deployment events.
[**list_external_cluster_machine_deployment_metrics**](ProjectApi.md#list_external_cluster_machine_deployment_metrics) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments/{machinedeployment_id}/nodes/metrics | List an external cluster machine deployment metrics.
[**list_external_cluster_machine_deployment_nodes**](ProjectApi.md#list_external_cluster_machine_deployment_nodes) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments/{machinedeployment_id}/nodes | Gets an external cluster machine deployment nodes.
[**list_external_cluster_machine_deployments**](ProjectApi.md#list_external_cluster_machine_deployments) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments | Gets an external cluster machine deployments.
[**list_external_cluster_nodes**](ProjectApi.md#list_external_cluster_nodes) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/nodes | Gets an external cluster nodes.
[**list_external_cluster_nodes_metrics**](ProjectApi.md#list_external_cluster_nodes_metrics) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/nodesmetrics | Gets an external cluster nodes metrics.
[**list_external_clusters**](ProjectApi.md#list_external_clusters) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters | Lists external clusters for the specified project.
[**list_gke_clusters**](ProjectApi.md#list_gke_clusters) | **GET** /api/v2/projects/{project_id}/providers/gke/clusters | Lists GKE clusters.
[**list_group_project_binding**](ProjectApi.md#list_group_project_binding) | **GET** /api/v2/projects/{project_id}/groupbindings | Lists project's group bindings.
[**list_machine_deployment_nodes**](ProjectApi.md#list_machine_deployment_nodes) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments/{machinedeployment_id}/nodes | Lists nodes that belong to the given machine deployment.
[**list_machine_deployment_nodes_events**](ProjectApi.md#list_machine_deployment_nodes_events) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments/{machinedeployment_id}/nodes/events | Lists machine deployment events. If query parameter `type` is set to `warning` then only warning events are retrieved.
[**list_machine_deployments**](ProjectApi.md#list_machine_deployments) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments | 
[**list_namespace**](ProjectApi.md#list_namespace) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/namespaces | 
[**list_namespace_v2**](ProjectApi.md#list_namespace_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/namespaces | 
[**list_node_deployment_nodes**](ProjectApi.md#list_node_deployment_nodes) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodedeployments/{nodedeployment_id}/nodes | Lists nodes that belong to the given node deployment.
[**list_node_deployment_nodes_events**](ProjectApi.md#list_node_deployment_nodes_events) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodedeployments/{nodedeployment_id}/nodes/events | Lists node deployment events. If query parameter `type` is set to `warning` then only warning events are retrieved.
[**list_node_deployments**](ProjectApi.md#list_node_deployments) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodedeployments | 
[**list_nodes_for_cluster**](ProjectApi.md#list_nodes_for_cluster) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/nodes | This endpoint is used for kubeadm cluster.
[**list_projects**](ProjectApi.md#list_projects) | **GET** /api/v1/projects | Lists projects that an authenticated user is a member of.
[**list_role**](ProjectApi.md#list_role) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/roles | 
[**list_role_binding**](ProjectApi.md#list_role_binding) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/bindings | 
[**list_role_binding_v2**](ProjectApi.md#list_role_binding_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/bindings | 
[**list_role_names**](ProjectApi.md#list_role_names) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/rolenames | 
[**list_role_names_v2**](ProjectApi.md#list_role_names_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/rolenames | 
[**list_role_v2**](ProjectApi.md#list_role_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/roles | 
[**list_ssh_keys**](ProjectApi.md#list_ssh_keys) | **GET** /api/v1/projects/{project_id}/sshkeys | Lists SSH Keys that belong to the given project.
[**list_ssh_keys_assigned_to_cluster**](ProjectApi.md#list_ssh_keys_assigned_to_cluster) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/sshkeys | 
[**list_ssh_keys_assigned_to_cluster_v2**](ProjectApi.md#list_ssh_keys_assigned_to_cluster_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/sshkeys | 
[**patch_cluster**](ProjectApi.md#patch_cluster) | **PATCH** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id} | Patches the given cluster using JSON Merge Patch method (https://tools.ietf.org/html/rfc7396).
[**patch_cluster_role**](ProjectApi.md#patch_cluster_role) | **PATCH** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/clusterroles/{role_id} | 
[**patch_cluster_v2**](ProjectApi.md#patch_cluster_v2) | **PATCH** /api/v2/projects/{project_id}/clusters/{cluster_id} | Patches the given cluster using JSON Merge Patch method (https://tools.ietf.org/html/rfc7396).
[**patch_constraint**](ProjectApi.md#patch_constraint) | **PATCH** /api/v2/projects/{project_id}/clusters/{cluster_id}/constraints/{constraint_name} | Patches a given constraint for the specified cluster.
[**patch_external_cluster**](ProjectApi.md#patch_external_cluster) | **PATCH** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id} | Patches the given cluster using JSON Merge Patch method (https://tools.ietf.org/html/rfc7396).
[**patch_external_cluster_machine_deployments**](ProjectApi.md#patch_external_cluster_machine_deployments) | **PATCH** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/machinedeployments/{machinedeployment_id} | 
[**patch_gatekeeper_config**](ProjectApi.md#patch_gatekeeper_config) | **PATCH** /api/v2/projects/{project_id}/clusters/{cluster_id}/gatekeeper/config | Patches the gatekeeper config for the specified cluster.
[**patch_group_project_binding**](ProjectApi.md#patch_group_project_binding) | **PATCH** /api/v2/projects/{project_id}/groupbindings/{binding_name} | Patch project group binding.
[**patch_machine_deployment**](ProjectApi.md#patch_machine_deployment) | **PATCH** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments/{machinedeployment_id} | 
[**patch_node_deployment**](ProjectApi.md#patch_node_deployment) | **PATCH** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodedeployments/{nodedeployment_id} | 
[**patch_role**](ProjectApi.md#patch_role) | **PATCH** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/roles/{namespace}/{role_id} | 
[**reset_alertmanager**](ProjectApi.md#reset_alertmanager) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/alertmanager/config | Resets the alertmanager configuration to default for the specified cluster.
[**restart_machine_deployment**](ProjectApi.md#restart_machine_deployment) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments/{machinedeployment_id} | Schedules rolling restart of a machine deployment that is assigned to the given cluster.
[**revoke_cluster_admin_token**](ProjectApi.md#revoke_cluster_admin_token) | **PUT** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/token | 
[**revoke_cluster_admin_token_v2**](ProjectApi.md#revoke_cluster_admin_token_v2) | **PUT** /api/v2/projects/{project_id}/clusters/{cluster_id}/token | 
[**revoke_cluster_viewer_token**](ProjectApi.md#revoke_cluster_viewer_token) | **PUT** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/viewertoken | 
[**revoke_cluster_viewer_token_v2**](ProjectApi.md#revoke_cluster_viewer_token_v2) | **PUT** /api/v2/projects/{project_id}/clusters/{cluster_id}/viewertoken | 
[**unbind_user_from_cluster_role_binding**](ProjectApi.md#unbind_user_from_cluster_role_binding) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/clusterroles/{role_id}/clusterbindings | 
[**unbind_user_from_cluster_role_binding_v2**](ProjectApi.md#unbind_user_from_cluster_role_binding_v2) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/clusterroles/{role_id}/clusterbindings | 
[**unbind_user_from_role_binding**](ProjectApi.md#unbind_user_from_role_binding) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/roles/{namespace}/{role_id}/bindings | 
[**unbind_user_from_role_binding_v2**](ProjectApi.md#unbind_user_from_role_binding_v2) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/roles/{namespace}/{role_id}/bindings | 
[**update_alertmanager**](ProjectApi.md#update_alertmanager) | **PUT** /api/v2/projects/{project_id}/clusters/{cluster_id}/alertmanager/config | 
[**update_external_cluster**](ProjectApi.md#update_external_cluster) | **PUT** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id} | Updates an external cluster for the given project.
[**update_project**](ProjectApi.md#update_project) | **PUT** /api/v1/projects/{project_id} | 
[**upgrade_cluster_node_deployments**](ProjectApi.md#upgrade_cluster_node_deployments) | **PUT** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodes/upgrades | 
[**upgrade_cluster_node_deployments_v2**](ProjectApi.md#upgrade_cluster_node_deployments_v2) | **PUT** /api/v2/projects/{project_id}/clusters/{cluster_id}/nodes/upgrades | 



## assign_ssh_key_to_cluster

> crate::models::SshKey assign_ssh_key_to_cluster(project_id, dc, cluster_id, key_id)


Assigns an existing ssh key to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

[**crate::models::SshKey**](SSHKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_ssh_key_to_cluster_v2

> crate::models::SshKey assign_ssh_key_to_cluster_v2(project_id, cluster_id, key_id)


Assigns an existing ssh key to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

[**crate::models::SshKey**](SSHKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bind_user_to_cluster_role

> crate::models::ClusterRoleBinding bind_user_to_cluster_role(project_id, dc, cluster_id, role_id, body)


Binds user to cluster role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**body** | Option<[**ClusterRoleUser**](ClusterRoleUser.md)> |  |  |

### Return type

[**crate::models::ClusterRoleBinding**](ClusterRoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bind_user_to_cluster_role_v2

> crate::models::ClusterRoleBinding bind_user_to_cluster_role_v2(project_id, cluster_id, role_id, body)


Binds user to cluster role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**body** | Option<[**ClusterRoleUser**](ClusterRoleUser.md)> |  |  |

### Return type

[**crate::models::ClusterRoleBinding**](ClusterRoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bind_user_to_role

> crate::models::RoleBinding bind_user_to_role(project_id, dc, cluster_id, role_id, namespace, body)


Binds user to the role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**RoleUser**](RoleUser.md)> |  |  |

### Return type

[**crate::models::RoleBinding**](RoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bind_user_to_role_v2

> crate::models::RoleBinding bind_user_to_role_v2(project_id, cluster_id, role_id, namespace, body)


Binds user to the role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**RoleUser**](RoleUser.md)> |  |  |

### Return type

[**crate::models::RoleBinding**](RoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cluster

> crate::models::Cluster create_cluster(project_id, dc, body)
Creates a cluster for the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**body** | Option<[**CreateClusterSpec**](CreateClusterSpec.md)> |  |  |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cluster_role

> crate::models::ClusterRole create_cluster_role(project_id, dc, cluster_id, body)


Creates cluster role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**ClusterRole**](ClusterRole.md)> |  |  |

### Return type

[**crate::models::ClusterRole**](ClusterRole.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cluster_template

> crate::models::ClusterTemplate create_cluster_template(project_id, body)
Creates a cluster templates for the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | Option<[**InlineObject8**](InlineObject8.md)> |  |  |

### Return type

[**crate::models::ClusterTemplate**](ClusterTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cluster_template_instance

> crate::models::ClusterTemplateInstance create_cluster_template_instance(project_id, template_id, body)
Create cluster template instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**template_id** | **String** |  | [required] |
**body** | Option<[**InlineObject10**](InlineObject10.md)> |  |  |

### Return type

[**crate::models::ClusterTemplateInstance**](ClusterTemplateInstance.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cluster_v2

> crate::models::Cluster create_cluster_v2(project_id, body)
Creates a cluster for the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | Option<[**CreateClusterSpec**](CreateClusterSpec.md)> |  |  |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_constraint

> crate::models::Constraint create_constraint(project_id, cluster_id, body)
Creates a given constraint for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | [**ConstraintBody**](ConstraintBody.md) |  | [required] |

### Return type

[**crate::models::Constraint**](Constraint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_external_cluster

> crate::models::ExternalCluster create_external_cluster(project_id, credential, body)
Creates an external cluster for the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**credential** | Option<**String**> | The credential name used in the preset for the provider |  |
**body** | Option<[**Body**](Body.md)> |  |  |

### Return type

[**crate::models::ExternalCluster**](ExternalCluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_external_cluster_machine_deployment

> crate::models::ExternalClusterMachineDeployment create_external_cluster_machine_deployment(project_id, cluster_id, body)
Create an external cluster machine deployments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**ExternalClusterMachineDeployment**](ExternalClusterMachineDeployment.md)> |  |  |

### Return type

[**crate::models::ExternalClusterMachineDeployment**](ExternalClusterMachineDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_gatekeeper_config

> crate::models::GatekeeperConfig create_gatekeeper_config(project_id, cluster_id, body)


Creates a gatekeeper config for the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | [**GatekeeperConfig**](GatekeeperConfig.md) |  | [required] |

### Return type

[**crate::models::GatekeeperConfig**](GatekeeperConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_project_binding

> crate::models::GroupProjectBinding create_group_project_binding(project_id, body)
Create project group binding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | [**GroupProjectBindingBody**](GroupProjectBindingBody.md) |  | [required] |

### Return type

[**crate::models::GroupProjectBinding**](GroupProjectBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_machine_deployment

> crate::models::NodeDeployment create_machine_deployment(project_id, cluster_id, body)


Creates a machine deployment that will belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**NodeDeployment**](NodeDeployment.md)> |  |  |

### Return type

[**crate::models::NodeDeployment**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_node_deployment

> crate::models::NodeDeployment create_node_deployment(project_id, dc, cluster_id, body)


Creates a node deployment that will belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**NodeDeployment**](NodeDeployment.md)> |  |  |

### Return type

[**crate::models::NodeDeployment**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project

> crate::models::Project create_project(body)
Creates a brand new project.

Note that this endpoint can be consumed by every authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**InlineObject4**](InlineObject4.md)> |  |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> crate::models::Role create_role(project_id, dc, cluster_id, body)


Creates cluster role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**Role**](Role.md)> |  |  |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ssh_key

> crate::models::SshKey create_ssh_key(project_id, key)
Adds the given SSH key to the specified project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**key** | Option<[**SshKey**](SshKey.md)> |  |  |

### Return type

[**crate::models::SshKey**](SSHKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster

> delete_cluster(project_id, dc, cluster_id, delete_volumes, delete_load_balancers)


Deletes the specified cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**delete_volumes** | Option<**bool**> |  |  |
**delete_load_balancers** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster_role

> delete_cluster_role(project_id, dc, cluster_id, role_id)


Delete the cluster role with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster_template

> delete_cluster_template(project_id, template_id)
Delete cluster template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**template_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster_v2

> delete_cluster_v2(project_id, cluster_id, delete_volumes, delete_load_balancers)


Deletes the specified cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**delete_volumes** | Option<**bool**> |  |  |
**delete_load_balancers** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_constraint

> delete_constraint(project_id, cluster_id, constraint_name)
Deletes a specified constraint for the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**constraint_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_external_cluster

> delete_external_cluster(project_id, cluster_id, action)


Deletes the specified external cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**action** | Option<**String**> | The Action is used to check if to `Delete` the cluster: both the actual cluter from the provider and the respective KKP cluster object By default the cluster will `Disconnect` which means the KKP cluster object will be deleted, cluster still exists on the provider, but is no longer connected/imported in KKP |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_external_cluster_machine_deployment

> delete_external_cluster_machine_deployment(project_id, cluster_id, machinedeployment_id)
Delete an external cluster machine deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_gatekeeper_config

> delete_gatekeeper_config(project_id, cluster_id)
Deletes the gatekeeper sync config for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_project_binding

> delete_group_project_binding(project_id, binding_name)
Delete project group binding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**binding_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_machine_deployment

> delete_machine_deployment(project_id, cluster_id, machinedeployment_id)
Deletes the given machine deployment that belongs to the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_machine_deployment_node

> delete_machine_deployment_node(project_id, cluster_id, node_id)
Deletes the given node that belongs to the machine deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_node_deployment

> delete_node_deployment(project_id, dc, cluster_id, nodedeployment_id)
Deletes the given node deployment that belongs to the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**nodedeployment_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_node_for_cluster_legacy

> delete_node_for_cluster_legacy(project_id, dc, cluster_id, node_id)
Deprecated: Deletes the given node that belongs to the cluster.

This endpoint is deprecated, please create a Node Deployment instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> delete_project(project_id)
Deletes the project with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> delete_role(project_id, dc, cluster_id, role_id, namespace)


Delete the cluster role with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ssh_key

> delete_ssh_key(project_id, key_id)
Removes the given SSH Key from the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_ssh_key_from_cluster

> detach_ssh_key_from_cluster(project_id, dc, key_id, cluster_id)


Unassignes an ssh key from the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**key_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_ssh_key_from_cluster_v2

> detach_ssh_key_from_cluster_v2(project_id, cluster_id, key_id)


Unassignes an ssh key from the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_cluster_template

> crate::models::ClusterTemplate export_cluster_template(project_id, template_id, format)
Export cluster template to file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**template_id** | **String** |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::ClusterTemplate**](ClusterTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alertmanager

> crate::models::Alertmanager get_alertmanager(project_id, cluster_id)
Gets the alertmanager configuration for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::Alertmanager**](Alertmanager.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backup_destination_names

> Vec<String> get_backup_destination_names(project_id, cluster_id)


Gets possible backup destination names for a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster

> crate::models::Cluster get_cluster(project_id, dc, cluster_id)


Gets the cluster with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_events

> Vec<crate::models::Event> get_cluster_events(project_id, dc, cluster_id, _type)
Gets the events related to the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_events_v2

> Vec<crate::models::Event> get_cluster_events_v2(project_id, cluster_id, _type)
Gets the events related to the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_health

> crate::models::ClusterHealth get_cluster_health(project_id, dc, cluster_id)


Returns the cluster's component health status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::ClusterHealth**](ClusterHealth.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_health_v2

> crate::models::ClusterHealth get_cluster_health_v2(project_id, cluster_id)


Returns the cluster's component health status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::ClusterHealth**](ClusterHealth.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_kubeconfig

> Vec<i32> get_cluster_kubeconfig(project_id, dc, cluster_id)
Gets the kubeconfig for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_kubeconfig_v2

> Vec<i32> get_cluster_kubeconfig_v2(project_id, cluster_id)
Gets the kubeconfig for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_metrics

> crate::models::ClusterMetrics get_cluster_metrics(project_id, dc, cluster_id)


Gets cluster metrics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::ClusterMetrics**](ClusterMetrics.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_metrics_v2

> crate::models::ClusterMetrics get_cluster_metrics_v2(project_id, cluster_id)


Gets cluster metrics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::ClusterMetrics**](ClusterMetrics.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_oidc

> crate::models::OidcSpec get_cluster_oidc(project_id, cluster_id)
Gets the OIDC params for the specified cluster with OIDC authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::OidcSpec**](OIDCSpec.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_role

> crate::models::ClusterRole get_cluster_role(project_id, dc, cluster_id, role_id)


Gets the cluster role with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |

### Return type

[**crate::models::ClusterRole**](ClusterRole.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_template

> crate::models::ClusterTemplate get_cluster_template(project_id, template_id)
Get cluster template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**template_id** | **String** |  | [required] |

### Return type

[**crate::models::ClusterTemplate**](ClusterTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_upgrades

> Vec<crate::models::MasterVersion> get_cluster_upgrades(project_id, dc, cluster_id)


Gets possible cluster upgrades

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::MasterVersion>**](MasterVersion.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_upgrades_v2

> Vec<crate::models::MasterVersion> get_cluster_upgrades_v2(project_id, cluster_id)


Gets possible cluster upgrades

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::MasterVersion>**](MasterVersion.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_v2

> crate::models::Cluster get_cluster_v2(project_id, cluster_id)


Gets the cluster with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_constraint

> crate::models::Constraint get_constraint(project_id, cluster_id, constraint_name)
Gets an specified constraint for the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**constraint_name** | **String** |  | [required] |

### Return type

[**crate::models::Constraint**](Constraint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_cluster

> crate::models::ExternalCluster get_external_cluster(project_id, cluster_id)
Gets an external cluster for the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::ExternalCluster**](ExternalCluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_cluster_kubeconfig

> Vec<i32> get_external_cluster_kubeconfig(project_id, cluster_id)
Gets the kubeconfig for the specified external cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_cluster_machine_deployment

> crate::models::ExternalClusterMachineDeployment get_external_cluster_machine_deployment(project_id, cluster_id, machinedeployment_id)
Gets an external cluster machine deployments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

[**crate::models::ExternalClusterMachineDeployment**](ExternalClusterMachineDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_cluster_machine_deployment_upgrades

> Vec<crate::models::MasterVersion> get_external_cluster_machine_deployment_upgrades(project_id, cluster_id, machinedeployment_id)
Gets an external cluster machine deployments upgrade versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::MasterVersion>**](MasterVersion.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_cluster_metrics

> crate::models::ClusterMetrics get_external_cluster_metrics(project_id, cluster_id)


Gets cluster metrics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::ClusterMetrics**](ClusterMetrics.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_cluster_node

> crate::models::ExternalClusterNode get_external_cluster_node(project_id, cluster_id, node_id)
Gets an external cluster node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

[**crate::models::ExternalClusterNode**](ExternalClusterNode.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_cluster_upgrades

> Vec<crate::models::MasterVersion> get_external_cluster_upgrades(project_id, cluster_id)
Gets an external cluster upgrades.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::MasterVersion>**](MasterVersion.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gatekeeper_config

> crate::models::GatekeeperConfig get_gatekeeper_config(project_id, cluster_id)
Gets the gatekeeper sync config for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::GatekeeperConfig**](GatekeeperConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_project_binding

> crate::models::GroupProjectBinding get_group_project_binding(project_id, binding_name)
Get project group binding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**binding_name** | **String** |  | [required] |

### Return type

[**crate::models::GroupProjectBinding**](GroupProjectBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_machine_deployment

> crate::models::NodeDeployment get_machine_deployment(project_id, cluster_id, machinedeployment_id)
Gets a machine deployment that is assigned to the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

[**crate::models::NodeDeployment**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_deployment

> crate::models::NodeDeployment get_node_deployment(project_id, dc, cluster_id, nodedeployment_id)
Gets a node deployment that is assigned to the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**nodedeployment_id** | **String** |  | [required] |

### Return type

[**crate::models::NodeDeployment**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oidc_cluster_kubeconfig

> Vec<i32> get_oidc_cluster_kubeconfig(project_id, dc, cluster_id)
Gets the kubeconfig for the specified cluster with oidc authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oidc_cluster_kubeconfig_v2

> Vec<i32> get_oidc_cluster_kubeconfig_v2(project_id, cluster_id)
Gets the kubeconfig for the specified cluster with oidc authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> crate::models::Project get_project(project_id)


Gets the project with the given ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_quota

> crate::models::ResourceQuota get_project_quota(project_id)
Returns Resource Quota for a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**crate::models::ResourceQuota**](ResourceQuota.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> crate::models::Role get_role(project_id, dc, cluster_id, role_id, namespace)


Gets the role with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_cluster_template

> crate::models::ClusterTemplate import_cluster_template(project_id, body)
Import a cluster templates for the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | Option<[**InlineObject9**](InlineObject9.md)> |  |  |

### Return type

[**crate::models::ClusterTemplate**](ClusterTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aks_clusters

> Vec<crate::models::AksCluster> list_aks_clusters(project_id, tenant_id, subscription_id, client_id, client_secret, credential)


Lists AKS clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**subscription_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AksCluster>**](AKSCluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_role

> Vec<crate::models::ClusterRole> list_cluster_role(project_id, dc, cluster_id)


Lists all ClusterRoles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ClusterRole>**](ClusterRole.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_role_binding

> Vec<crate::models::ClusterRoleBinding> list_cluster_role_binding(project_id, dc, cluster_id)


List cluster role binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ClusterRoleBinding>**](ClusterRoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_role_binding_v2

> Vec<crate::models::ClusterRoleBinding> list_cluster_role_binding_v2(project_id, cluster_id)


List cluster role binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ClusterRoleBinding>**](ClusterRoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_role_names

> Vec<crate::models::ClusterRoleName> list_cluster_role_names(project_id, dc, cluster_id)


Lists all ClusterRoles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ClusterRoleName>**](ClusterRoleName.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_role_names_v2

> Vec<crate::models::ClusterRoleName> list_cluster_role_names_v2(project_id, cluster_id)


Lists all ClusterRoles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ClusterRoleName>**](ClusterRoleName.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_role_v2

> Vec<crate::models::ClusterRole> list_cluster_role_v2(project_id, cluster_id)


Lists all ClusterRoles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ClusterRole>**](ClusterRole.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_templates

> Vec<crate::models::ClusterTemplate> list_cluster_templates(project_id)
List cluster templates for the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ClusterTemplate>**](ClusterTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_clusters

> Vec<crate::models::Cluster> list_clusters(project_id, dc)
Lists clusters for the specified project and data center.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Cluster>**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_clusters_for_project

> Vec<crate::models::Cluster> list_clusters_for_project(project_id)
Lists clusters for the specified project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Cluster>**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_clusters_v2

> Vec<crate::models::Cluster> list_clusters_v2(project_id, show_dm_count)
Lists clusters for the specified project. If query parameter `show_dm_count` is set to `true` then the endpoint will also return the number of machine deployments of each cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**show_dm_count** | Option<**bool**> |  |  |

### Return type

[**Vec<crate::models::Cluster>**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cni_plugin_versions_for_cluster

> crate::models::CniVersions list_cni_plugin_versions_for_cluster(project_id, cluster_id)
Lists CNI plugin versions for a given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::CniVersions**](CNIVersions.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_constraints

> Vec<crate::models::Constraint> list_constraints(project_id, cluster_id)
Lists constraints for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Constraint>**](Constraint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eks_clusters

> Vec<crate::models::EksCluster> list_eks_clusters(project_id, access_key_id, secret_access_key, credential, region)


Lists EKS clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**access_key_id** | Option<**String**> |  |  |
**secret_access_key** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EksCluster>**](EKSCluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_external_cluster_events

> Vec<crate::models::Event> list_external_cluster_events(project_id, cluster_id, _type)
Gets an external cluster events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_external_cluster_machine_deployment_events

> Vec<crate::models::Event> list_external_cluster_machine_deployment_events(project_id, cluster_id, machinedeployment_id, _type)
List an external cluster machine deployment events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_external_cluster_machine_deployment_metrics

> Vec<crate::models::NodeMetric> list_external_cluster_machine_deployment_metrics(project_id, cluster_id, machinedeployment_id)
List an external cluster machine deployment metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NodeMetric>**](NodeMetric.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_external_cluster_machine_deployment_nodes

> Vec<crate::models::ExternalClusterNode> list_external_cluster_machine_deployment_nodes(project_id, cluster_id, machinedeployment_id)
Gets an external cluster machine deployment nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ExternalClusterNode>**](ExternalClusterNode.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_external_cluster_machine_deployments

> Vec<crate::models::ExternalClusterMachineDeployment> list_external_cluster_machine_deployments(project_id, cluster_id)
Gets an external cluster machine deployments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ExternalClusterMachineDeployment>**](ExternalClusterMachineDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_external_cluster_nodes

> Vec<crate::models::ExternalClusterNode> list_external_cluster_nodes(project_id, cluster_id)
Gets an external cluster nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ExternalClusterNode>**](ExternalClusterNode.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_external_cluster_nodes_metrics

> Vec<crate::models::NodeMetric> list_external_cluster_nodes_metrics(project_id, cluster_id)
Gets an external cluster nodes metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NodeMetric>**](NodeMetric.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_external_clusters

> Vec<crate::models::ExternalCluster> list_external_clusters(project_id)
Lists external clusters for the specified project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ExternalCluster>**](ExternalCluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gke_clusters

> Vec<crate::models::GkeCluster> list_gke_clusters(project_id, service_account, credential)
Lists GKE clusters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**service_account** | Option<**String**> | The plain GCP service account |  |
**credential** | Option<**String**> | The credential name used in the preset for the GCP provider |  |

### Return type

[**Vec<crate::models::GkeCluster>**](GKECluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_project_binding

> Vec<crate::models::GroupProjectBinding> list_group_project_binding(project_id)
Lists project's group bindings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::GroupProjectBinding>**](GroupProjectBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_machine_deployment_nodes

> Vec<crate::models::Node> list_machine_deployment_nodes(project_id, cluster_id, machinedeployment_id, hide_initial_conditions)
Lists nodes that belong to the given machine deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |
**hide_initial_conditions** | Option<**bool**> |  |  |

### Return type

[**Vec<crate::models::Node>**](Node.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_machine_deployment_nodes_events

> Vec<crate::models::Event> list_machine_deployment_nodes_events(project_id, cluster_id, machinedeployment_id, _type)
Lists machine deployment events. If query parameter `type` is set to `warning` then only warning events are retrieved.

If the value is 'normal' then normal events are returned. If the query parameter is missing method returns all events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_machine_deployments

> Vec<crate::models::NodeDeployment> list_machine_deployments(project_id, cluster_id)


Lists machine deployments that belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NodeDeployment>**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespace

> Vec<crate::models::Namespace> list_namespace(project_id, dc, cluster_id)


Lists all namespaces in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Namespace>**](Namespace.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespace_v2

> Vec<crate::models::Namespace> list_namespace_v2(project_id, cluster_id)


Lists all namespaces in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Namespace>**](Namespace.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_node_deployment_nodes

> Vec<crate::models::Node> list_node_deployment_nodes(project_id, dc, cluster_id, nodedeployment_id, hide_initial_conditions)
Lists nodes that belong to the given node deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**nodedeployment_id** | **String** |  | [required] |
**hide_initial_conditions** | Option<**bool**> |  |  |

### Return type

[**Vec<crate::models::Node>**](Node.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_node_deployment_nodes_events

> Vec<crate::models::Event> list_node_deployment_nodes_events(project_id, dc, cluster_id, nodedeployment_id, _type)
Lists node deployment events. If query parameter `type` is set to `warning` then only warning events are retrieved.

If the value is 'normal' then normal events are returned. If the query parameter is missing method returns all events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**nodedeployment_id** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_node_deployments

> Vec<crate::models::NodeDeployment> list_node_deployments(project_id, dc, cluster_id)


Lists node deployments that belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NodeDeployment>**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nodes_for_cluster

> Vec<crate::models::Node> list_nodes_for_cluster(project_id, cluster_id, hide_initial_conditions)
This endpoint is used for kubeadm cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**hide_initial_conditions** | Option<**bool**> |  |  |

### Return type

[**Vec<crate::models::Node>**](Node.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_projects

> Vec<crate::models::Project> list_projects(display_all)
Lists projects that an authenticated user is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**display_all** | Option<**bool**> |  |  |

### Return type

[**Vec<crate::models::Project>**](Project.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role

> Vec<crate::models::Role> list_role(project_id, dc, cluster_id)


Lists all Roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role_binding

> Vec<crate::models::RoleBinding> list_role_binding(project_id, dc, cluster_id)


List role binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::RoleBinding>**](RoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role_binding_v2

> Vec<crate::models::RoleBinding> list_role_binding_v2(project_id, cluster_id)


List role binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::RoleBinding>**](RoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role_names

> Vec<crate::models::RoleName> list_role_names(project_id, dc, cluster_id)


Lists all Role names with namespaces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::RoleName>**](RoleName.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role_names_v2

> Vec<crate::models::RoleName> list_role_names_v2(project_id, cluster_id)


Lists all Role names with namespaces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::RoleName>**](RoleName.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role_v2

> Vec<crate::models::Role> list_role_v2(project_id, cluster_id)


Lists all Roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ssh_keys

> Vec<crate::models::SshKey> list_ssh_keys(project_id)
Lists SSH Keys that belong to the given project.

The returned collection is sorted by creation timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::SshKey>**](SSHKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ssh_keys_assigned_to_cluster

> Vec<crate::models::SshKey> list_ssh_keys_assigned_to_cluster(project_id, dc, cluster_id)


Lists ssh keys that are assigned to the cluster The returned collection is sorted by creation timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::SshKey>**](SSHKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ssh_keys_assigned_to_cluster_v2

> Vec<crate::models::SshKey> list_ssh_keys_assigned_to_cluster_v2(project_id, cluster_id)


Lists ssh keys that are assigned to the cluster The returned collection is sorted by creation timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::SshKey>**](SSHKey.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_cluster

> crate::models::Cluster patch_cluster(project_id, dc, cluster_id, patch)
Patches the given cluster using JSON Merge Patch method (https://tools.ietf.org/html/rfc7396).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_cluster_role

> crate::models::ClusterRole patch_cluster_role(project_id, dc, cluster_id, role_id, patch)


Patch the cluster role with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::ClusterRole**](ClusterRole.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_cluster_v2

> crate::models::Cluster patch_cluster_v2(project_id, cluster_id, patch)
Patches the given cluster using JSON Merge Patch method (https://tools.ietf.org/html/rfc7396).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_constraint

> crate::models::Constraint patch_constraint(project_id, cluster_id, constraint_name, patch)
Patches a given constraint for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**constraint_name** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::Constraint**](Constraint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_external_cluster

> crate::models::ExternalCluster patch_external_cluster(project_id, cluster_id, patch)
Patches the given cluster using JSON Merge Patch method (https://tools.ietf.org/html/rfc7396).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::ExternalCluster**](ExternalCluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_external_cluster_machine_deployments

> crate::models::ExternalClusterMachineDeployment patch_external_cluster_machine_deployments(project_id, cluster_id, machinedeployment_id, patch)


Patches the given cluster using JSON Merge Patch method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::ExternalClusterMachineDeployment**](ExternalClusterMachineDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_gatekeeper_config

> crate::models::GatekeeperConfig patch_gatekeeper_config(project_id, cluster_id, patch)
Patches the gatekeeper config for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::GatekeeperConfig**](GatekeeperConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_group_project_binding

> crate::models::GroupProjectBinding patch_group_project_binding(project_id, binding_name, body)
Patch project group binding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**binding_name** | **String** |  | [required] |
**body** | [**InlineObject11**](InlineObject11.md) |  | [required] |

### Return type

[**crate::models::GroupProjectBinding**](GroupProjectBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_machine_deployment

> crate::models::NodeDeployment patch_machine_deployment(project_id, cluster_id, machinedeployment_id, patch)


Patches a machine deployment that is assigned to the given cluster. Please note that at the moment only node deployment's spec can be updated by a patch, no other fields can be changed using this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::NodeDeployment**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_node_deployment

> crate::models::NodeDeployment patch_node_deployment(project_id, dc, cluster_id, nodedeployment_id, patch)


Patches a node deployment that is assigned to the given cluster. Please note that at the moment only node deployment's spec can be updated by a patch, no other fields can be changed using this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**nodedeployment_id** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::NodeDeployment**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_role

> crate::models::Role patch_role(project_id, dc, cluster_id, role_id, namespace, patch)


Patch the role with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_alertmanager

> reset_alertmanager(project_id, cluster_id)
Resets the alertmanager configuration to default for the specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_machine_deployment

> crate::models::NodeDeployment restart_machine_deployment(project_id, cluster_id, machinedeployment_id)
Schedules rolling restart of a machine deployment that is assigned to the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

[**crate::models::NodeDeployment**](NodeDeployment.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_cluster_admin_token

> revoke_cluster_admin_token(project_id, dc, cluster_id)


Revokes the current admin token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_cluster_admin_token_v2

> revoke_cluster_admin_token_v2(project_id, cluster_id)


Revokes the current admin token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_cluster_viewer_token

> revoke_cluster_viewer_token(project_id, dc, cluster_id)


Revokes the current viewer token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_cluster_viewer_token_v2

> revoke_cluster_viewer_token_v2(project_id, cluster_id)


Revokes the current viewer token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unbind_user_from_cluster_role_binding

> crate::models::ClusterRoleBinding unbind_user_from_cluster_role_binding(project_id, dc, cluster_id, role_id, body)


Unbinds user from cluster role binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**body** | Option<[**ClusterRoleUser**](ClusterRoleUser.md)> |  |  |

### Return type

[**crate::models::ClusterRoleBinding**](ClusterRoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unbind_user_from_cluster_role_binding_v2

> crate::models::ClusterRoleBinding unbind_user_from_cluster_role_binding_v2(project_id, cluster_id, role_id, body)


Unbinds user from cluster role binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**body** | Option<[**ClusterRoleUser**](ClusterRoleUser.md)> |  |  |

### Return type

[**crate::models::ClusterRoleBinding**](ClusterRoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unbind_user_from_role_binding

> crate::models::RoleBinding unbind_user_from_role_binding(project_id, dc, cluster_id, role_id, namespace, body)


Unbinds user from the role binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**RoleUser**](RoleUser.md)> |  |  |

### Return type

[**crate::models::RoleBinding**](RoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unbind_user_from_role_binding_v2

> crate::models::RoleBinding unbind_user_from_role_binding_v2(project_id, cluster_id, role_id, namespace, body)


Unbinds user from the role binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**RoleUser**](RoleUser.md)> |  |  |

### Return type

[**crate::models::RoleBinding**](RoleBinding.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alertmanager

> crate::models::Alertmanager update_alertmanager(project_id, cluster_id, body)


Updates an alertmanager configuration for the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | [**Alertmanager**](Alertmanager.md) |  | [required] |

### Return type

[**crate::models::Alertmanager**](Alertmanager.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_external_cluster

> crate::models::ExternalCluster update_external_cluster(project_id, cluster_id, body)
Updates an external cluster for the given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**InlineObject12**](InlineObject12.md)> |  |  |

### Return type

[**crate::models::ExternalCluster**](ExternalCluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> crate::models::Project update_project(project_id, body)


Updates the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_cluster_node_deployments

> upgrade_cluster_node_deployments(project_id, dc, cluster_id, body)


Upgrades node deployments in a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**MasterVersion**](MasterVersion.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_cluster_node_deployments_v2

> upgrade_cluster_node_deployments_v2(project_id, cluster_id, body)


Upgrades node deployments in a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**MasterVersion**](MasterVersion.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

