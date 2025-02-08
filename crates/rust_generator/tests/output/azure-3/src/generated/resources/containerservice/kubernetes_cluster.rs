/// Manages a Managed Kubernetes Cluster (also known as AKS / Azure Kubernetes Service)
///
/// ## Example Usage
///
/// This example provisions a basic Managed Kubernetes Cluster.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example-aks1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       dnsPrefix: exampleaks1
///       defaultNodePool:
///         name: default
///         nodeCount: 1
///         vmSize: Standard_D2_v2
///       identity:
///         type: SystemAssigned
///       tags:
///         Environment: Production
/// outputs:
///   clientCertificate: ${exampleKubernetesCluster.kubeConfigs[0].clientCertificate}
///   kubeConfig: ${exampleKubernetesCluster.kubeConfigRaw}
/// ```
///
/// ## Import
///
/// Managed Kubernetes Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/kubernetesCluster:KubernetesCluster cluster1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ContainerService/managedClusters/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod kubernetes_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KubernetesClusterArgs {
        /// A `aci_connector_linux` block as defined below. For more details, please visit [Create and configure an AKS cluster to use virtual nodes](https://docs.microsoft.com/azure/aks/virtual-nodes-portal).
        #[builder(into, default)]
        pub aci_connector_linux: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterAciConnectorLinux,
            >,
        >,
        /// An `api_server_access_profile` block as defined below.
        #[builder(into, default)]
        pub api_server_access_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterApiServerAccessProfile,
            >,
        >,
        /// A `auto_scaler_profile` block as defined below.
        #[builder(into, default)]
        pub auto_scaler_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterAutoScalerProfile,
            >,
        >,
        /// The upgrade channel for this Kubernetes Cluster. Possible values are `patch`, `rapid`, `node-image` and `stable`. Omitting this field sets this value to `none`.
        ///
        /// !> **Note:** Cluster Auto-Upgrade will update the Kubernetes Cluster (and its Node Pools) to the latest GA version of Kubernetes automatically - please [see the Azure documentation for more information](https://docs.microsoft.com/azure/aks/upgrade-cluster#set-auto-upgrade-channel).
        ///
        /// > **Note:** Cluster Auto-Upgrade only updates to GA versions of Kubernetes and will not update to Preview versions.
        #[builder(into, default)]
        pub automatic_upgrade_channel: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `azure_active_directory_role_based_access_control` block as defined below.
        #[builder(into, default)]
        pub azure_active_directory_role_based_access_control: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterAzureActiveDirectoryRoleBasedAccessControl,
            >,
        >,
        /// Should the Azure Policy Add-On be enabled? For more details please visit [Understand Azure Policy for Azure Kubernetes Service](https://docs.microsoft.com/en-ie/azure/governance/policy/concepts/rego-for-aks)
        #[builder(into, default)]
        pub azure_policy_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `confidential_computing` block as defined below. For more details please [the documentation](https://learn.microsoft.com/en-us/azure/confidential-computing/confidential-nodes-aks-overview)
        #[builder(into, default)]
        pub confidential_computing: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterConfidentialComputing,
            >,
        >,
        /// Should cost analysis be enabled for this Kubernetes Cluster? Defaults to `false`. The `sku_tier` must be set to `Standard` or `Premium` to enable this feature. Enabling this will add Kubernetes Namespace and Deployment details to the Cost Analysis views in the Azure portal.
        #[builder(into, default)]
        pub cost_analysis_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies configuration for "System" mode node pool. A `default_node_pool` block as defined below.
        #[builder(into)]
        pub default_node_pool: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::containerservice::KubernetesClusterDefaultNodePool,
        >,
        /// The ID of the Disk Encryption Set which should be used for the Nodes and Volumes. More information [can be found in the documentation](https://docs.microsoft.com/azure/aks/azure-disk-customer-managed-keys). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub disk_encryption_set_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DNS prefix specified when creating the managed cluster. Possible values must begin and end with a letter or number, contain only letters, numbers, and hyphens and be between 1 and 54 characters in length. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dns_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the DNS prefix to use with private clusters. Changing this forces a new resource to be created.
        ///
        /// > **Note:** You must define either a `dns_prefix` or a `dns_prefix_private_cluster` field.
        ///
        /// In addition, one of either `identity` or `service_principal` blocks must be specified.
        #[builder(into, default)]
        pub dns_prefix_private_cluster: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the Extended Zone (formerly called Edge Zone) within the Azure Region where this Managed Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should HTTP Application Routing be enabled?
        ///
        /// > **Note:** At this time HTTP Application Routing is not supported in Azure China or Azure US Government.
        #[builder(into, default)]
        pub http_application_routing_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `http_proxy_config` block as defined below.
        #[builder(into, default)]
        pub http_proxy_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterHttpProxyConfig,
            >,
        >,
        /// An `identity` block as defined below. One of either `identity` or `service_principal` must be specified.
        ///
        /// !> **Note:** A migration scenario from `service_principal` to `identity` is supported. When upgrading `service_principal` to `identity`, your cluster's control plane and addon pods will switch to use managed identity, but the kubelets will keep using your configured `service_principal` until you upgrade your Node Pool.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::KubernetesClusterIdentity>,
        >,
        /// Specifies whether Image Cleaner is enabled.
        #[builder(into, default)]
        pub image_cleaner_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the interval in hours when images should be cleaned up. Defaults to `0`.
        #[builder(into, default)]
        pub image_cleaner_interval_hours: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// A `ingress_application_gateway` block as defined below.
        ///
        /// > **Note:** Since the Application Gateway is deployed inside a Virtual Network, users (and Service Principals) that are operating the Application Gateway must have the `Microsoft.Network/virtualNetworks/subnets/join/action` permission on the Virtual Network or Subnet. For more details, please visit [Virtual Network Permission](https://learn.microsoft.com/en-us/azure/application-gateway/configuration-infrastructure#virtual-network-permission).
        #[builder(into, default)]
        pub ingress_application_gateway: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterIngressApplicationGateway,
            >,
        >,
        /// A `key_management_service` block as defined below. For more details, please visit [Key Management Service (KMS) etcd encryption to an AKS cluster](https://learn.microsoft.com/en-us/azure/aks/use-kms-etcd-encryption).
        #[builder(into, default)]
        pub key_management_service: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterKeyManagementService,
            >,
        >,
        /// A `key_vault_secrets_provider` block as defined below. For more details, please visit [Azure Keyvault Secrets Provider for AKS](https://docs.microsoft.com/azure/aks/csi-secrets-store-driver).
        #[builder(into, default)]
        pub key_vault_secrets_provider: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterKeyVaultSecretsProvider,
            >,
        >,
        /// A `kubelet_identity` block as defined below.
        #[builder(into, default)]
        pub kubelet_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterKubeletIdentity,
            >,
        >,
        /// Version of Kubernetes specified when creating the AKS managed cluster. If not specified, the latest recommended version will be used at provisioning time (but won't auto-upgrade). AKS does not require an exact patch version to be specified, minor version aliases such as `1.22` are also supported. - The minor version's latest GA patch is automatically chosen in that case. More details can be found in [the documentation](https://docs.microsoft.com/en-us/azure/aks/supported-kubernetes-versions?tabs=azure-cli#alias-minor-version).
        ///
        /// > **Note:** Upgrading your cluster may take up to 10 minutes per node.
        #[builder(into, default)]
        pub kubernetes_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `linux_profile` block as defined below.
        #[builder(into, default)]
        pub linux_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::KubernetesClusterLinuxProfile>,
        >,
        /// If `true` local accounts will be disabled. See [the documentation](https://docs.microsoft.com/azure/aks/managed-aad#disable-local-accounts) for more information.
        ///
        /// > **Note:** If `local_account_disabled` is set to `true`, it is required to enable Kubernetes RBAC and AKS-managed Azure AD integration. See [the documentation](https://docs.microsoft.com/azure/aks/managed-aad#azure-ad-authentication-overview) for more information.
        #[builder(into, default)]
        pub local_account_disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The location where the Managed Kubernetes Cluster should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `maintenance_window` block as defined below.
        #[builder(into, default)]
        pub maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterMaintenanceWindow,
            >,
        >,
        /// A `maintenance_window_auto_upgrade` block as defined below.
        #[builder(into, default)]
        pub maintenance_window_auto_upgrade: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterMaintenanceWindowAutoUpgrade,
            >,
        >,
        /// A `maintenance_window_node_os` block as defined below.
        #[builder(into, default)]
        pub maintenance_window_node_os: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterMaintenanceWindowNodeOs,
            >,
        >,
        /// A `microsoft_defender` block as defined below.
        #[builder(into, default)]
        pub microsoft_defender: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterMicrosoftDefender,
            >,
        >,
        /// Specifies a Prometheus add-on profile for the Kubernetes Cluster. A `monitor_metrics` block as defined below.
        ///
        /// > **Note:** If deploying Managed Prometheus, the `monitor_metrics` properties are required to configure the cluster for metrics collection. If no value is needed, set properties to `null`.
        #[builder(into, default)]
        pub monitor_metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterMonitorMetrics,
            >,
        >,
        /// The name of the Managed Kubernetes Cluster to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network_profile` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If `network_profile` is not defined, `kubenet` profile will be used by default.
        #[builder(into, default)]
        pub network_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterNetworkProfile,
            >,
        >,
        /// The upgrade channel for this Kubernetes Cluster Nodes' OS Image. Possible values are `Unmanaged`, `SecurityPatch`, `NodeImage` and `None`. Defaults to `NodeImage`.
        ///
        /// > **Note:** `node_os_upgrade_channel` must be set to `NodeImage` if `automatic_upgrade_channel` has been set to `node-image`
        #[builder(into, default)]
        pub node_os_upgrade_channel: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Kubernetes Nodes should exist. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Azure requires that a new, non-existent Resource Group is used, as otherwise, the provisioning of the Kubernetes Service will fail.
        #[builder(into, default)]
        pub node_resource_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable or Disable the [OIDC issuer URL](https://learn.microsoft.com/en-gb/azure/aks/use-oidc-issuer)
        #[builder(into, default)]
        pub oidc_issuer_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `oms_agent` block as defined below.
        #[builder(into, default)]
        pub oms_agent: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::KubernetesClusterOmsAgent>,
        >,
        /// Is Open Service Mesh enabled? For more details, please visit [Open Service Mesh for AKS](https://docs.microsoft.com/azure/aks/open-service-mesh-about).
        #[builder(into, default)]
        pub open_service_mesh_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should this Kubernetes Cluster have its API server only exposed on internal IP addresses? This provides a Private IP Address for the Kubernetes API on the Virtual Network where the Kubernetes Cluster is located. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub private_cluster_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether a Public FQDN for this Private Cluster should be added. Defaults to `false`.
        ///
        /// > **Note:** If you use BYO DNS Zone, the AKS cluster should either use a User Assigned Identity or a service principal (which is deprecated) with the `Private DNS Zone Contributor` role and access to this Private DNS Zone. If `UserAssigned` identity is used - to prevent improper resource order destruction - the cluster should depend on the role assignment, like in this example:
        ///
        /// ```ignore
        /// use pulumi_gestalt_rust::Output;
        /// use pulumi_gestalt_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let example = resource_group::create(
        ///         "example",
        ///         ResourceGroupArgs::builder()
        ///             .location("West Europe")
        ///             .name("example")
        ///             .build_struct(),
        ///     );
        ///     let exampleAssignment = assignment::create(
        ///         "exampleAssignment",
        ///         AssignmentArgs::builder()
        ///             .principal_id("${exampleUserAssignedIdentity.principalId}")
        ///             .role_definition_name("Private DNS Zone Contributor")
        ///             .scope("${exampleZone.id}")
        ///             .build_struct(),
        ///     );
        ///     let exampleKubernetesCluster = kubernetes_cluster::create(
        ///         "exampleKubernetesCluster",
        ///         KubernetesClusterArgs::builder()
        ///             .dns_prefix("aksexamplednsprefix1")
        ///             .location("${example.location}")
        ///             .name("aksexamplewithprivatednszone1")
        ///             .private_cluster_enabled(true)
        ///             .private_dns_zone_id("${exampleZone.id}")
        ///             .resource_group_name("${example.name}")
        ///             .build_struct(),
        ///     );
        ///     let exampleUserAssignedIdentity = user_assigned_identity::create(
        ///         "exampleUserAssignedIdentity",
        ///         UserAssignedIdentityArgs::builder()
        ///             .location("${example.location}")
        ///             .name("aks-example-identity")
        ///             .resource_group_name("${example.name}")
        ///             .build_struct(),
        ///     );
        ///     let exampleZone = zone::create(
        ///         "exampleZone",
        ///         ZoneArgs::builder()
        ///             .name("privatelink.eastus2.azmk8s.io")
        ///             .resource_group_name("${example.name}")
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        #[builder(into, default)]
        pub private_cluster_public_fqdn_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Either the ID of Private DNS Zone which should be delegated to this Cluster, `System` to have AKS manage this or `None`. In case of `None` you will need to bring your own DNS server and set up resolving, otherwise, the cluster will have issues after provisioning. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub private_dns_zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Resource Group where the Managed Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether Role Based Access Control for the Kubernetes Cluster should be enabled. Defaults to `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub role_based_access_control_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to enable run command for the cluster or not. Defaults to `true`.
        #[builder(into, default)]
        pub run_command_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `service_mesh_profile` block as defined below.
        #[builder(into, default)]
        pub service_mesh_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterServiceMeshProfile,
            >,
        >,
        /// A `service_principal` block as documented below. One of either `identity` or `service_principal` must be specified.
        ///
        /// !> **Note:** A migration scenario from `service_principal` to `identity` is supported. When upgrading `service_principal` to `identity`, your cluster's control plane and addon pods will switch to use managed identity, but the kubelets will keep using your configured `service_principal` until you upgrade your Node Pool.
        #[builder(into, default)]
        pub service_principal: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterServicePrincipal,
            >,
        >,
        /// The SKU Tier that should be used for this Kubernetes Cluster. Possible values are `Free`, `Standard` (which includes the Uptime SLA) and `Premium`. Defaults to `Free`.
        ///
        /// > **Note:** Whilst the AKS API previously supported the `Paid` SKU - the AKS API introduced a breaking change in API Version `2023-02-01` (used in v3.51.0 and later) where the value `Paid` must now be set to `Standard`.
        #[builder(into, default)]
        pub sku_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `storage_profile` block as defined below.
        #[builder(into, default)]
        pub storage_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterStorageProfile,
            >,
        >,
        /// Specifies the support plan which should be used for this Kubernetes Cluster. Possible values are `KubernetesOfficial` and `AKSLongTermSupport`. Defaults to `KubernetesOfficial`.
        #[builder(into, default)]
        pub support_plan: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `web_app_routing` block as defined below.
        #[builder(into, default)]
        pub web_app_routing: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::KubernetesClusterWebAppRouting>,
        >,
        /// A `windows_profile` block as defined below.
        #[builder(into, default)]
        pub windows_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterWindowsProfile,
            >,
        >,
        /// A `workload_autoscaler_profile` block defined below.
        #[builder(into, default)]
        pub workload_autoscaler_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterWorkloadAutoscalerProfile,
            >,
        >,
        /// Specifies whether Azure AD Workload Identity should be enabled for the Cluster. Defaults to `false`.
        ///
        /// > **Note:** To enable Azure AD Workload Identity `oidc_issuer_enabled` must be set to `true`.
        ///
        /// > **Note:** Enabling this option will allocate Workload Identity resources to the `kube-system` namespace in Kubernetes. If you wish to customize the deployment of Workload Identity, you can refer to [the documentation on Azure AD Workload Identity.](https://azure.github.io/azure-workload-identity/docs/installation/mutating-admission-webhook.html) The documentation provides guidance on how to install the mutating admission webhook, which allows for the customization of Workload Identity deployment.
        #[builder(into, default)]
        pub workload_identity_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct KubernetesClusterResult {
        /// A `aci_connector_linux` block as defined below. For more details, please visit [Create and configure an AKS cluster to use virtual nodes](https://docs.microsoft.com/azure/aks/virtual-nodes-portal).
        pub aci_connector_linux: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterAciConnectorLinux,
            >,
        >,
        /// An `api_server_access_profile` block as defined below.
        pub api_server_access_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterApiServerAccessProfile,
            >,
        >,
        /// A `auto_scaler_profile` block as defined below.
        pub auto_scaler_profile: pulumi_gestalt_rust::Output<
            super::super::types::containerservice::KubernetesClusterAutoScalerProfile,
        >,
        /// The upgrade channel for this Kubernetes Cluster. Possible values are `patch`, `rapid`, `node-image` and `stable`. Omitting this field sets this value to `none`.
        ///
        /// !> **Note:** Cluster Auto-Upgrade will update the Kubernetes Cluster (and its Node Pools) to the latest GA version of Kubernetes automatically - please [see the Azure documentation for more information](https://docs.microsoft.com/azure/aks/upgrade-cluster#set-auto-upgrade-channel).
        ///
        /// > **Note:** Cluster Auto-Upgrade only updates to GA versions of Kubernetes and will not update to Preview versions.
        pub automatic_upgrade_channel: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `azure_active_directory_role_based_access_control` block as defined below.
        pub azure_active_directory_role_based_access_control: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterAzureActiveDirectoryRoleBasedAccessControl,
            >,
        >,
        /// Should the Azure Policy Add-On be enabled? For more details please visit [Understand Azure Policy for Azure Kubernetes Service](https://docs.microsoft.com/en-ie/azure/governance/policy/concepts/rego-for-aks)
        pub azure_policy_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `confidential_computing` block as defined below. For more details please [the documentation](https://learn.microsoft.com/en-us/azure/confidential-computing/confidential-nodes-aks-overview)
        pub confidential_computing: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterConfidentialComputing,
            >,
        >,
        /// Should cost analysis be enabled for this Kubernetes Cluster? Defaults to `false`. The `sku_tier` must be set to `Standard` or `Premium` to enable this feature. Enabling this will add Kubernetes Namespace and Deployment details to the Cost Analysis views in the Azure portal.
        pub cost_analysis_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The current version running on the Azure Kubernetes Managed Cluster.
        pub current_kubernetes_version: pulumi_gestalt_rust::Output<String>,
        /// Specifies configuration for "System" mode node pool. A `default_node_pool` block as defined below.
        pub default_node_pool: pulumi_gestalt_rust::Output<
            super::super::types::containerservice::KubernetesClusterDefaultNodePool,
        >,
        /// The ID of the Disk Encryption Set which should be used for the Nodes and Volumes. More information [can be found in the documentation](https://docs.microsoft.com/azure/aks/azure-disk-customer-managed-keys). Changing this forces a new resource to be created.
        pub disk_encryption_set_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// DNS prefix specified when creating the managed cluster. Possible values must begin and end with a letter or number, contain only letters, numbers, and hyphens and be between 1 and 54 characters in length. Changing this forces a new resource to be created.
        pub dns_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the DNS prefix to use with private clusters. Changing this forces a new resource to be created.
        ///
        /// > **Note:** You must define either a `dns_prefix` or a `dns_prefix_private_cluster` field.
        ///
        /// In addition, one of either `identity` or `service_principal` blocks must be specified.
        pub dns_prefix_private_cluster: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Extended Zone (formerly called Edge Zone) within the Azure Region where this Managed Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The FQDN of the Azure Kubernetes Managed Cluster.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// Should HTTP Application Routing be enabled?
        ///
        /// > **Note:** At this time HTTP Application Routing is not supported in Azure China or Azure US Government.
        pub http_application_routing_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Zone Name of the HTTP Application Routing.
        pub http_application_routing_zone_name: pulumi_gestalt_rust::Output<String>,
        /// A `http_proxy_config` block as defined below.
        pub http_proxy_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterHttpProxyConfig,
            >,
        >,
        /// An `identity` block as defined below. One of either `identity` or `service_principal` must be specified.
        ///
        /// !> **Note:** A migration scenario from `service_principal` to `identity` is supported. When upgrading `service_principal` to `identity`, your cluster's control plane and addon pods will switch to use managed identity, but the kubelets will keep using your configured `service_principal` until you upgrade your Node Pool.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::KubernetesClusterIdentity>,
        >,
        /// Specifies whether Image Cleaner is enabled.
        pub image_cleaner_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the interval in hours when images should be cleaned up. Defaults to `0`.
        pub image_cleaner_interval_hours: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A `ingress_application_gateway` block as defined below.
        ///
        /// > **Note:** Since the Application Gateway is deployed inside a Virtual Network, users (and Service Principals) that are operating the Application Gateway must have the `Microsoft.Network/virtualNetworks/subnets/join/action` permission on the Virtual Network or Subnet. For more details, please visit [Virtual Network Permission](https://learn.microsoft.com/en-us/azure/application-gateway/configuration-infrastructure#virtual-network-permission).
        pub ingress_application_gateway: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterIngressApplicationGateway,
            >,
        >,
        /// A `key_management_service` block as defined below. For more details, please visit [Key Management Service (KMS) etcd encryption to an AKS cluster](https://learn.microsoft.com/en-us/azure/aks/use-kms-etcd-encryption).
        pub key_management_service: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterKeyManagementService,
            >,
        >,
        /// A `key_vault_secrets_provider` block as defined below. For more details, please visit [Azure Keyvault Secrets Provider for AKS](https://docs.microsoft.com/azure/aks/csi-secrets-store-driver).
        pub key_vault_secrets_provider: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterKeyVaultSecretsProvider,
            >,
        >,
        /// Raw Kubernetes config for the admin account to be used by [kubectl](https://kubernetes.io/docs/reference/kubectl/overview/) and other compatible tools. This is only available when Role Based Access Control with Azure Active Directory is enabled and local accounts enabled.
        pub kube_admin_config_raw: pulumi_gestalt_rust::Output<String>,
        /// A `kube_admin_config` block as defined below. This is only available when Role Based Access Control with Azure Active Directory is enabled and local accounts enabled.
        pub kube_admin_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::containerservice::KubernetesClusterKubeAdminConfig>,
        >,
        /// Raw Kubernetes config to be used by [kubectl](https://kubernetes.io/docs/reference/kubectl/overview/) and other compatible tools.
        pub kube_config_raw: pulumi_gestalt_rust::Output<String>,
        /// A `kube_config` block as defined below.
        pub kube_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::containerservice::KubernetesClusterKubeConfig>,
        >,
        /// A `kubelet_identity` block as defined below.
        pub kubelet_identity: pulumi_gestalt_rust::Output<
            super::super::types::containerservice::KubernetesClusterKubeletIdentity,
        >,
        /// Version of Kubernetes specified when creating the AKS managed cluster. If not specified, the latest recommended version will be used at provisioning time (but won't auto-upgrade). AKS does not require an exact patch version to be specified, minor version aliases such as `1.22` are also supported. - The minor version's latest GA patch is automatically chosen in that case. More details can be found in [the documentation](https://docs.microsoft.com/en-us/azure/aks/supported-kubernetes-versions?tabs=azure-cli#alias-minor-version).
        ///
        /// > **Note:** Upgrading your cluster may take up to 10 minutes per node.
        pub kubernetes_version: pulumi_gestalt_rust::Output<String>,
        /// A `linux_profile` block as defined below.
        pub linux_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::KubernetesClusterLinuxProfile>,
        >,
        /// If `true` local accounts will be disabled. See [the documentation](https://docs.microsoft.com/azure/aks/managed-aad#disable-local-accounts) for more information.
        ///
        /// > **Note:** If `local_account_disabled` is set to `true`, it is required to enable Kubernetes RBAC and AKS-managed Azure AD integration. See [the documentation](https://docs.microsoft.com/azure/aks/managed-aad#azure-ad-authentication-overview) for more information.
        pub local_account_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The location where the Managed Kubernetes Cluster should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `maintenance_window` block as defined below.
        pub maintenance_window: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterMaintenanceWindow,
            >,
        >,
        /// A `maintenance_window_auto_upgrade` block as defined below.
        pub maintenance_window_auto_upgrade: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterMaintenanceWindowAutoUpgrade,
            >,
        >,
        /// A `maintenance_window_node_os` block as defined below.
        pub maintenance_window_node_os: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterMaintenanceWindowNodeOs,
            >,
        >,
        /// A `microsoft_defender` block as defined below.
        pub microsoft_defender: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterMicrosoftDefender,
            >,
        >,
        /// Specifies a Prometheus add-on profile for the Kubernetes Cluster. A `monitor_metrics` block as defined below.
        ///
        /// > **Note:** If deploying Managed Prometheus, the `monitor_metrics` properties are required to configure the cluster for metrics collection. If no value is needed, set properties to `null`.
        pub monitor_metrics: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterMonitorMetrics,
            >,
        >,
        /// The name of the Managed Kubernetes Cluster to create. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network_profile` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If `network_profile` is not defined, `kubenet` profile will be used by default.
        pub network_profile: pulumi_gestalt_rust::Output<
            super::super::types::containerservice::KubernetesClusterNetworkProfile,
        >,
        /// The upgrade channel for this Kubernetes Cluster Nodes' OS Image. Possible values are `Unmanaged`, `SecurityPatch`, `NodeImage` and `None`. Defaults to `NodeImage`.
        ///
        /// > **Note:** `node_os_upgrade_channel` must be set to `NodeImage` if `automatic_upgrade_channel` has been set to `node-image`
        pub node_os_upgrade_channel: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Kubernetes Nodes should exist. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Azure requires that a new, non-existent Resource Group is used, as otherwise, the provisioning of the Kubernetes Service will fail.
        pub node_resource_group: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Resource Group containing the resources for this Managed Kubernetes Cluster.
        pub node_resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// Enable or Disable the [OIDC issuer URL](https://learn.microsoft.com/en-gb/azure/aks/use-oidc-issuer)
        pub oidc_issuer_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The OIDC issuer URL that is associated with the cluster.
        pub oidc_issuer_url: pulumi_gestalt_rust::Output<String>,
        /// A `oms_agent` block as defined below.
        pub oms_agent: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::KubernetesClusterOmsAgent>,
        >,
        /// Is Open Service Mesh enabled? For more details, please visit [Open Service Mesh for AKS](https://docs.microsoft.com/azure/aks/open-service-mesh-about).
        pub open_service_mesh_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The FQDN for the Azure Portal resources when private link has been enabled, which is only resolvable inside the Virtual Network used by the Kubernetes Cluster.
        pub portal_fqdn: pulumi_gestalt_rust::Output<String>,
        /// Should this Kubernetes Cluster have its API server only exposed on internal IP addresses? This provides a Private IP Address for the Kubernetes API on the Virtual Network where the Kubernetes Cluster is located. Defaults to `false`. Changing this forces a new resource to be created.
        pub private_cluster_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether a Public FQDN for this Private Cluster should be added. Defaults to `false`.
        ///
        /// > **Note:** If you use BYO DNS Zone, the AKS cluster should either use a User Assigned Identity or a service principal (which is deprecated) with the `Private DNS Zone Contributor` role and access to this Private DNS Zone. If `UserAssigned` identity is used - to prevent improper resource order destruction - the cluster should depend on the role assignment, like in this example:
        ///
        /// ```ignore
        /// use pulumi_gestalt_rust::Output;
        /// use pulumi_gestalt_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let example = resource_group::create(
        ///         "example",
        ///         ResourceGroupArgs::builder()
        ///             .location("West Europe")
        ///             .name("example")
        ///             .build_struct(),
        ///     );
        ///     let exampleAssignment = assignment::create(
        ///         "exampleAssignment",
        ///         AssignmentArgs::builder()
        ///             .principal_id("${exampleUserAssignedIdentity.principalId}")
        ///             .role_definition_name("Private DNS Zone Contributor")
        ///             .scope("${exampleZone.id}")
        ///             .build_struct(),
        ///     );
        ///     let exampleKubernetesCluster = kubernetes_cluster::create(
        ///         "exampleKubernetesCluster",
        ///         KubernetesClusterArgs::builder()
        ///             .dns_prefix("aksexamplednsprefix1")
        ///             .location("${example.location}")
        ///             .name("aksexamplewithprivatednszone1")
        ///             .private_cluster_enabled(true)
        ///             .private_dns_zone_id("${exampleZone.id}")
        ///             .resource_group_name("${example.name}")
        ///             .build_struct(),
        ///     );
        ///     let exampleUserAssignedIdentity = user_assigned_identity::create(
        ///         "exampleUserAssignedIdentity",
        ///         UserAssignedIdentityArgs::builder()
        ///             .location("${example.location}")
        ///             .name("aks-example-identity")
        ///             .resource_group_name("${example.name}")
        ///             .build_struct(),
        ///     );
        ///     let exampleZone = zone::create(
        ///         "exampleZone",
        ///         ZoneArgs::builder()
        ///             .name("privatelink.eastus2.azmk8s.io")
        ///             .resource_group_name("${example.name}")
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        pub private_cluster_public_fqdn_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Either the ID of Private DNS Zone which should be delegated to this Cluster, `System` to have AKS manage this or `None`. In case of `None` you will need to bring your own DNS server and set up resolving, otherwise, the cluster will have issues after provisioning. Changing this forces a new resource to be created.
        pub private_dns_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The FQDN for the Kubernetes Cluster when private link has been enabled, which is only resolvable inside the Virtual Network used by the Kubernetes Cluster.
        pub private_fqdn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Resource Group where the Managed Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Whether Role Based Access Control for the Kubernetes Cluster should be enabled. Defaults to `true`. Changing this forces a new resource to be created.
        pub role_based_access_control_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to enable run command for the cluster or not. Defaults to `true`.
        pub run_command_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `service_mesh_profile` block as defined below.
        pub service_mesh_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterServiceMeshProfile,
            >,
        >,
        /// A `service_principal` block as documented below. One of either `identity` or `service_principal` must be specified.
        ///
        /// !> **Note:** A migration scenario from `service_principal` to `identity` is supported. When upgrading `service_principal` to `identity`, your cluster's control plane and addon pods will switch to use managed identity, but the kubelets will keep using your configured `service_principal` until you upgrade your Node Pool.
        pub service_principal: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterServicePrincipal,
            >,
        >,
        /// The SKU Tier that should be used for this Kubernetes Cluster. Possible values are `Free`, `Standard` (which includes the Uptime SLA) and `Premium`. Defaults to `Free`.
        ///
        /// > **Note:** Whilst the AKS API previously supported the `Paid` SKU - the AKS API introduced a breaking change in API Version `2023-02-01` (used in v3.51.0 and later) where the value `Paid` must now be set to `Standard`.
        pub sku_tier: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `storage_profile` block as defined below.
        pub storage_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterStorageProfile,
            >,
        >,
        /// Specifies the support plan which should be used for this Kubernetes Cluster. Possible values are `KubernetesOfficial` and `AKSLongTermSupport`. Defaults to `KubernetesOfficial`.
        pub support_plan: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `web_app_routing` block as defined below.
        pub web_app_routing: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::KubernetesClusterWebAppRouting>,
        >,
        /// A `windows_profile` block as defined below.
        pub windows_profile: pulumi_gestalt_rust::Output<
            super::super::types::containerservice::KubernetesClusterWindowsProfile,
        >,
        /// A `workload_autoscaler_profile` block defined below.
        pub workload_autoscaler_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterWorkloadAutoscalerProfile,
            >,
        >,
        /// Specifies whether Azure AD Workload Identity should be enabled for the Cluster. Defaults to `false`.
        ///
        /// > **Note:** To enable Azure AD Workload Identity `oidc_issuer_enabled` must be set to `true`.
        ///
        /// > **Note:** Enabling this option will allocate Workload Identity resources to the `kube-system` namespace in Kubernetes. If you wish to customize the deployment of Workload Identity, you can refer to [the documentation on Azure AD Workload Identity.](https://azure.github.io/azure-workload-identity/docs/installation/mutating-admission-webhook.html) The documentation provides guidance on how to install the mutating admission webhook, which allows for the customization of Workload Identity deployment.
        pub workload_identity_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KubernetesClusterArgs,
    ) -> KubernetesClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let aci_connector_linux_binding = args
            .aci_connector_linux
            .get_output(context)
            .get_inner();
        let api_server_access_profile_binding = args
            .api_server_access_profile
            .get_output(context)
            .get_inner();
        let auto_scaler_profile_binding = args
            .auto_scaler_profile
            .get_output(context)
            .get_inner();
        let automatic_upgrade_channel_binding = args
            .automatic_upgrade_channel
            .get_output(context)
            .get_inner();
        let azure_active_directory_role_based_access_control_binding = args
            .azure_active_directory_role_based_access_control
            .get_output(context)
            .get_inner();
        let azure_policy_enabled_binding = args
            .azure_policy_enabled
            .get_output(context)
            .get_inner();
        let confidential_computing_binding = args
            .confidential_computing
            .get_output(context)
            .get_inner();
        let cost_analysis_enabled_binding = args
            .cost_analysis_enabled
            .get_output(context)
            .get_inner();
        let default_node_pool_binding = args
            .default_node_pool
            .get_output(context)
            .get_inner();
        let disk_encryption_set_id_binding = args
            .disk_encryption_set_id
            .get_output(context)
            .get_inner();
        let dns_prefix_binding = args.dns_prefix.get_output(context).get_inner();
        let dns_prefix_private_cluster_binding = args
            .dns_prefix_private_cluster
            .get_output(context)
            .get_inner();
        let edge_zone_binding = args.edge_zone.get_output(context).get_inner();
        let http_application_routing_enabled_binding = args
            .http_application_routing_enabled
            .get_output(context)
            .get_inner();
        let http_proxy_config_binding = args
            .http_proxy_config
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let image_cleaner_enabled_binding = args
            .image_cleaner_enabled
            .get_output(context)
            .get_inner();
        let image_cleaner_interval_hours_binding = args
            .image_cleaner_interval_hours
            .get_output(context)
            .get_inner();
        let ingress_application_gateway_binding = args
            .ingress_application_gateway
            .get_output(context)
            .get_inner();
        let key_management_service_binding = args
            .key_management_service
            .get_output(context)
            .get_inner();
        let key_vault_secrets_provider_binding = args
            .key_vault_secrets_provider
            .get_output(context)
            .get_inner();
        let kubelet_identity_binding = args
            .kubelet_identity
            .get_output(context)
            .get_inner();
        let kubernetes_version_binding = args
            .kubernetes_version
            .get_output(context)
            .get_inner();
        let linux_profile_binding = args.linux_profile.get_output(context).get_inner();
        let local_account_disabled_binding = args
            .local_account_disabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let maintenance_window_binding = args
            .maintenance_window
            .get_output(context)
            .get_inner();
        let maintenance_window_auto_upgrade_binding = args
            .maintenance_window_auto_upgrade
            .get_output(context)
            .get_inner();
        let maintenance_window_node_os_binding = args
            .maintenance_window_node_os
            .get_output(context)
            .get_inner();
        let microsoft_defender_binding = args
            .microsoft_defender
            .get_output(context)
            .get_inner();
        let monitor_metrics_binding = args
            .monitor_metrics
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_profile_binding = args
            .network_profile
            .get_output(context)
            .get_inner();
        let node_os_upgrade_channel_binding = args
            .node_os_upgrade_channel
            .get_output(context)
            .get_inner();
        let node_resource_group_binding = args
            .node_resource_group
            .get_output(context)
            .get_inner();
        let oidc_issuer_enabled_binding = args
            .oidc_issuer_enabled
            .get_output(context)
            .get_inner();
        let oms_agent_binding = args.oms_agent.get_output(context).get_inner();
        let open_service_mesh_enabled_binding = args
            .open_service_mesh_enabled
            .get_output(context)
            .get_inner();
        let private_cluster_enabled_binding = args
            .private_cluster_enabled
            .get_output(context)
            .get_inner();
        let private_cluster_public_fqdn_enabled_binding = args
            .private_cluster_public_fqdn_enabled
            .get_output(context)
            .get_inner();
        let private_dns_zone_id_binding = args
            .private_dns_zone_id
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let role_based_access_control_enabled_binding = args
            .role_based_access_control_enabled
            .get_output(context)
            .get_inner();
        let run_command_enabled_binding = args
            .run_command_enabled
            .get_output(context)
            .get_inner();
        let service_mesh_profile_binding = args
            .service_mesh_profile
            .get_output(context)
            .get_inner();
        let service_principal_binding = args
            .service_principal
            .get_output(context)
            .get_inner();
        let sku_tier_binding = args.sku_tier.get_output(context).get_inner();
        let storage_profile_binding = args
            .storage_profile
            .get_output(context)
            .get_inner();
        let support_plan_binding = args.support_plan.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let web_app_routing_binding = args
            .web_app_routing
            .get_output(context)
            .get_inner();
        let windows_profile_binding = args
            .windows_profile
            .get_output(context)
            .get_inner();
        let workload_autoscaler_profile_binding = args
            .workload_autoscaler_profile
            .get_output(context)
            .get_inner();
        let workload_identity_enabled_binding = args
            .workload_identity_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/kubernetesCluster:KubernetesCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aciConnectorLinux".into(),
                    value: &aci_connector_linux_binding,
                },
                register_interface::ObjectField {
                    name: "apiServerAccessProfile".into(),
                    value: &api_server_access_profile_binding,
                },
                register_interface::ObjectField {
                    name: "autoScalerProfile".into(),
                    value: &auto_scaler_profile_binding,
                },
                register_interface::ObjectField {
                    name: "automaticUpgradeChannel".into(),
                    value: &automatic_upgrade_channel_binding,
                },
                register_interface::ObjectField {
                    name: "azureActiveDirectoryRoleBasedAccessControl".into(),
                    value: &azure_active_directory_role_based_access_control_binding,
                },
                register_interface::ObjectField {
                    name: "azurePolicyEnabled".into(),
                    value: &azure_policy_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "confidentialComputing".into(),
                    value: &confidential_computing_binding,
                },
                register_interface::ObjectField {
                    name: "costAnalysisEnabled".into(),
                    value: &cost_analysis_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "defaultNodePool".into(),
                    value: &default_node_pool_binding,
                },
                register_interface::ObjectField {
                    name: "diskEncryptionSetId".into(),
                    value: &disk_encryption_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "dnsPrefix".into(),
                    value: &dns_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "dnsPrefixPrivateCluster".into(),
                    value: &dns_prefix_private_cluster_binding,
                },
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "httpApplicationRoutingEnabled".into(),
                    value: &http_application_routing_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "httpProxyConfig".into(),
                    value: &http_proxy_config_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "imageCleanerEnabled".into(),
                    value: &image_cleaner_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "imageCleanerIntervalHours".into(),
                    value: &image_cleaner_interval_hours_binding,
                },
                register_interface::ObjectField {
                    name: "ingressApplicationGateway".into(),
                    value: &ingress_application_gateway_binding,
                },
                register_interface::ObjectField {
                    name: "keyManagementService".into(),
                    value: &key_management_service_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultSecretsProvider".into(),
                    value: &key_vault_secrets_provider_binding,
                },
                register_interface::ObjectField {
                    name: "kubeletIdentity".into(),
                    value: &kubelet_identity_binding,
                },
                register_interface::ObjectField {
                    name: "kubernetesVersion".into(),
                    value: &kubernetes_version_binding,
                },
                register_interface::ObjectField {
                    name: "linuxProfile".into(),
                    value: &linux_profile_binding,
                },
                register_interface::ObjectField {
                    name: "localAccountDisabled".into(),
                    value: &local_account_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindowAutoUpgrade".into(),
                    value: &maintenance_window_auto_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindowNodeOs".into(),
                    value: &maintenance_window_node_os_binding,
                },
                register_interface::ObjectField {
                    name: "microsoftDefender".into(),
                    value: &microsoft_defender_binding,
                },
                register_interface::ObjectField {
                    name: "monitorMetrics".into(),
                    value: &monitor_metrics_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "nodeOsUpgradeChannel".into(),
                    value: &node_os_upgrade_channel_binding,
                },
                register_interface::ObjectField {
                    name: "nodeResourceGroup".into(),
                    value: &node_resource_group_binding,
                },
                register_interface::ObjectField {
                    name: "oidcIssuerEnabled".into(),
                    value: &oidc_issuer_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "omsAgent".into(),
                    value: &oms_agent_binding,
                },
                register_interface::ObjectField {
                    name: "openServiceMeshEnabled".into(),
                    value: &open_service_mesh_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "privateClusterEnabled".into(),
                    value: &private_cluster_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "privateClusterPublicFqdnEnabled".into(),
                    value: &private_cluster_public_fqdn_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsZoneId".into(),
                    value: &private_dns_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "roleBasedAccessControlEnabled".into(),
                    value: &role_based_access_control_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "runCommandEnabled".into(),
                    value: &run_command_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "serviceMeshProfile".into(),
                    value: &service_mesh_profile_binding,
                },
                register_interface::ObjectField {
                    name: "servicePrincipal".into(),
                    value: &service_principal_binding,
                },
                register_interface::ObjectField {
                    name: "skuTier".into(),
                    value: &sku_tier_binding,
                },
                register_interface::ObjectField {
                    name: "storageProfile".into(),
                    value: &storage_profile_binding,
                },
                register_interface::ObjectField {
                    name: "supportPlan".into(),
                    value: &support_plan_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "webAppRouting".into(),
                    value: &web_app_routing_binding,
                },
                register_interface::ObjectField {
                    name: "windowsProfile".into(),
                    value: &windows_profile_binding,
                },
                register_interface::ObjectField {
                    name: "workloadAutoscalerProfile".into(),
                    value: &workload_autoscaler_profile_binding,
                },
                register_interface::ObjectField {
                    name: "workloadIdentityEnabled".into(),
                    value: &workload_identity_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KubernetesClusterResult {
            aci_connector_linux: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aciConnectorLinux"),
            ),
            api_server_access_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiServerAccessProfile"),
            ),
            auto_scaler_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoScalerProfile"),
            ),
            automatic_upgrade_channel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticUpgradeChannel"),
            ),
            azure_active_directory_role_based_access_control: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureActiveDirectoryRoleBasedAccessControl"),
            ),
            azure_policy_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azurePolicyEnabled"),
            ),
            confidential_computing: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("confidentialComputing"),
            ),
            cost_analysis_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("costAnalysisEnabled"),
            ),
            current_kubernetes_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("currentKubernetesVersion"),
            ),
            default_node_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultNodePool"),
            ),
            disk_encryption_set_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskEncryptionSetId"),
            ),
            dns_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsPrefix"),
            ),
            dns_prefix_private_cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsPrefixPrivateCluster"),
            ),
            edge_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeZone"),
            ),
            fqdn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fqdn")),
            http_application_routing_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpApplicationRoutingEnabled"),
            ),
            http_application_routing_zone_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpApplicationRoutingZoneName"),
            ),
            http_proxy_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpProxyConfig"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            image_cleaner_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageCleanerEnabled"),
            ),
            image_cleaner_interval_hours: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageCleanerIntervalHours"),
            ),
            ingress_application_gateway: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingressApplicationGateway"),
            ),
            key_management_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyManagementService"),
            ),
            key_vault_secrets_provider: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultSecretsProvider"),
            ),
            kube_admin_config_raw: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubeAdminConfigRaw"),
            ),
            kube_admin_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubeAdminConfigs"),
            ),
            kube_config_raw: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubeConfigRaw"),
            ),
            kube_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubeConfigs"),
            ),
            kubelet_identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubeletIdentity"),
            ),
            kubernetes_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubernetesVersion"),
            ),
            linux_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linuxProfile"),
            ),
            local_account_disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localAccountDisabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maintenance_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceWindow"),
            ),
            maintenance_window_auto_upgrade: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceWindowAutoUpgrade"),
            ),
            maintenance_window_node_os: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceWindowNodeOs"),
            ),
            microsoft_defender: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("microsoftDefender"),
            ),
            monitor_metrics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitorMetrics"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkProfile"),
            ),
            node_os_upgrade_channel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeOsUpgradeChannel"),
            ),
            node_resource_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeResourceGroup"),
            ),
            node_resource_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeResourceGroupId"),
            ),
            oidc_issuer_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("oidcIssuerEnabled"),
            ),
            oidc_issuer_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("oidcIssuerUrl"),
            ),
            oms_agent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("omsAgent"),
            ),
            open_service_mesh_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("openServiceMeshEnabled"),
            ),
            portal_fqdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portalFqdn"),
            ),
            private_cluster_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateClusterEnabled"),
            ),
            private_cluster_public_fqdn_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateClusterPublicFqdnEnabled"),
            ),
            private_dns_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsZoneId"),
            ),
            private_fqdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateFqdn"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            role_based_access_control_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleBasedAccessControlEnabled"),
            ),
            run_command_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runCommandEnabled"),
            ),
            service_mesh_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceMeshProfile"),
            ),
            service_principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servicePrincipal"),
            ),
            sku_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuTier"),
            ),
            storage_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageProfile"),
            ),
            support_plan: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportPlan"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            web_app_routing: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("webAppRouting"),
            ),
            windows_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("windowsProfile"),
            ),
            workload_autoscaler_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadAutoscalerProfile"),
            ),
            workload_identity_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadIdentityEnabled"),
            ),
        }
    }
}
