/// Manages a Node Pool within a Kubernetes Cluster
///
/// > **NOTE:** Multiple Node Pools are only supported when the Kubernetes Cluster is using Virtual Machine Scale Sets.
///
/// ## Example Usage
///
/// This example provisions a basic Kubernetes Node Pool.
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
///       servicePrincipal:
///         clientId: 00000000-0000-0000-0000-000000000000
///         clientSecret: '00000000000000000000000000000000'
///   exampleKubernetesClusterNodePool:
///     type: azure:containerservice:KubernetesClusterNodePool
///     name: example
///     properties:
///       name: internal
///       kubernetesClusterId: ${exampleKubernetesCluster.id}
///       vmSize: Standard_DS2_v2
///       nodeCount: 1
///       tags:
///         Environment: Production
/// ```
///
/// ## Import
///
/// Kubernetes Cluster Node Pools can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/kubernetesClusterNodePool:KubernetesClusterNodePool pool1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ContainerService/managedClusters/cluster1/agentPools/pool1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kubernetes_cluster_node_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KubernetesClusterNodePoolArgs {
        /// Whether to enable [auto-scaler](https://docs.microsoft.com/azure/aks/cluster-autoscaler).
        #[builder(into, default)]
        pub auto_scaling_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ID of the Capacity Reservation Group where this Node Pool should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub capacity_reservation_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Eviction Policy which should be used for Virtual Machines within the Virtual Machine Scale Set powering this Node Pool. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** An Eviction Policy can only be configured when `priority` is set to `Spot` and will default to `Delete` unless otherwise specified.
        #[builder(into, default)]
        pub eviction_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the nodes in this Node Pool have Federal Information Processing Standard enabled? Changing this forces a new resource to be created.
        ///
        /// > **Note:** FIPS support is in Public Preview - more information and details on how to opt into the Preview can be found in [this article](https://docs.microsoft.com/azure/aks/use-multiple-node-pools#add-a-fips-enabled-node-pool-preview).
        #[builder(into, default)]
        pub fips_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the GPU MIG instance profile for supported GPU VM SKU. The allowed values are `MIG1g`, `MIG2g`, `MIG3g`, `MIG4g` and `MIG7g`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub gpu_instance: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the nodes in this Node Pool have host encryption enabled? Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Additional fields must be configured depending on the value of this field - see below.
        #[builder(into, default)]
        pub host_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The fully qualified resource ID of the Dedicated Host Group to provision virtual machines from. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub host_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `kubelet_config` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub kubelet_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolKubeletConfig,
            >,
        >,
        /// The type of disk used by kubelet. Possible values are `OS` and `Temporary`.
        #[builder(into, default)]
        pub kubelet_disk_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Kubernetes Cluster where this Node Pool should exist. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The type of Default Node Pool for the Kubernetes Cluster must be `VirtualMachineScaleSets` to attach multiple node pools.
        #[builder(into)]
        pub kubernetes_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `linux_os_config` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub linux_os_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolLinuxOsConfig,
            >,
        >,
        #[builder(into, default)]
        pub max_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum number of pods that can run on each agent. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub max_pods: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub min_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Should this Node Pool be used for System or User resources? Possible values are `System` and `User`. Defaults to `User`.
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Node Pool which should be created within the Kubernetes Cluster. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A Windows Node Pool cannot have a `name` longer than 6 characters.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub node_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A map of Kubernetes labels which should be applied to nodes in this Node Pool.
        #[builder(into, default)]
        pub node_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `node_network_profile` block as documented below.
        #[builder(into, default)]
        pub node_network_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolNodeNetworkProfile,
            >,
        >,
        /// Should each node have a Public IP Address? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub node_public_ip_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Resource ID for the Public IP Addresses Prefix for the nodes in this Node Pool. `node_public_ip_enabled` should be `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub node_public_ip_prefix_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Kubernetes taints which should be applied to nodes in the agent pool (e.g `key=value:NoSchedule`).
        #[builder(into, default)]
        pub node_taints: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Version of Kubernetes used for the Agents. If not specified, the latest recommended version will be used at provisioning time (but won't auto-upgrade). AKS does not require an exact patch version to be specified, minor version aliases such as `1.22` are also supported. - The minor version's latest GA patch is automatically chosen in that case. More details can be found in [the documentation](https://docs.microsoft.com/en-us/azure/aks/supported-kubernetes-versions?tabs=azure-cli#alias-minor-version).
        ///
        /// > **Note:** This version must be supported by the Kubernetes Cluster - as such the version of Kubernetes used on the Cluster/Control Plane may need to be upgraded first.
        #[builder(into, default)]
        pub orchestrator_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Agent Operating System disk size in GB. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub os_disk_size_gb: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The type of disk which should be used for the Operating System. Possible values are `Ephemeral` and `Managed`. Defaults to `Managed`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub os_disk_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the OS SKU used by the agent pool. Possible values are `AzureLinux`, `Ubuntu`, `Windows2019` and `Windows2022`. If not specified, the default is `Ubuntu` if OSType=Linux or `Windows2019` if OSType=Windows. And the default Windows OSSKU will be changed to `Windows2022` after Windows2019 is deprecated. Changing this from `AzureLinux` or `Ubuntu` to `AzureLinux` or `Ubuntu` will not replace the resource, otherwise it forces a new resource to be created.
        #[builder(into, default)]
        pub os_sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Operating System which should be used for this Node Pool. Changing this forces a new resource to be created. Possible values are `Linux` and `Windows`. Defaults to `Linux`.
        #[builder(into, default)]
        pub os_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Subnet where the pods in the Node Pool should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub pod_subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Priority for Virtual Machines within the Virtual Machine Scale Set that powers this Node Pool. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Proximity Placement Group where the Virtual Machine Scale Set that powers this Node Pool will be placed. Changing this forces a new resource to be created.
        ///
        /// > **Note:** When setting `priority` to Spot - you must configure an `eviction_policy`, `spot_max_price` and add the applicable `node_labels` and `node_taints` [as per the Azure Documentation](https://docs.microsoft.com/azure/aks/spot-node-pool).
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies how the node pool should deal with scaled-down nodes. Allowed values are `Delete` and `Deallocate`. Defaults to `Delete`.
        #[builder(into, default)]
        pub scale_down_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Snapshot which should be used to create this Node Pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub snapshot_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum price you're willing to pay in USD per Virtual Machine. Valid values are `-1` (the current on-demand price for a Virtual Machine) or a positive value with up to five decimal places. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This field can only be configured when `priority` is set to `Spot`.
        #[builder(into, default)]
        pub spot_max_price: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > At this time there's a bug in the AKS API where Tags for a Node Pool are not stored in the correct case - you may wish to use [`ignoreChanges`](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) functionality to ignore changes to the casing until this is fixed in the AKS API.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Used to specify whether the UltraSSD is enabled in the Node Pool. Defaults to `false`. See [the documentation](https://docs.microsoft.com/azure/aks/use-ultra-disks) for more information. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub ultra_ssd_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `upgrade_settings` block as documented below.
        #[builder(into, default)]
        pub upgrade_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolUpgradeSettings,
            >,
        >,
        /// The SKU which should be used for the Virtual Machines used in this Node Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vm_size: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet where this Node Pool should exist. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A route table must be configured on this Subnet.
        #[builder(into, default)]
        pub vnet_subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `windows_profile` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub windows_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolWindowsProfile,
            >,
        >,
        /// Used to specify the workload runtime. Allowed values are `OCIContainer` and `WasmWasi`.
        ///
        /// > **Note:** WebAssembly System Interface node pools are in Public Preview - more information and details on how to opt into the preview can be found in [this article](https://docs.microsoft.com/azure/aks/use-wasi-node-pools)
        #[builder(into, default)]
        pub workload_runtime: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Availability Zones in which this Kubernetes Cluster Node Pool should be located. Changing this forces a new Kubernetes Cluster Node Pool to be created.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct KubernetesClusterNodePoolResult {
        /// Whether to enable [auto-scaler](https://docs.microsoft.com/azure/aks/cluster-autoscaler).
        pub auto_scaling_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the ID of the Capacity Reservation Group where this Node Pool should exist. Changing this forces a new resource to be created.
        pub capacity_reservation_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Eviction Policy which should be used for Virtual Machines within the Virtual Machine Scale Set powering this Node Pool. Possible values are `Deallocate` and `Delete`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** An Eviction Policy can only be configured when `priority` is set to `Spot` and will default to `Delete` unless otherwise specified.
        pub eviction_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the nodes in this Node Pool have Federal Information Processing Standard enabled? Changing this forces a new resource to be created.
        ///
        /// > **Note:** FIPS support is in Public Preview - more information and details on how to opt into the Preview can be found in [this article](https://docs.microsoft.com/azure/aks/use-multiple-node-pools#add-a-fips-enabled-node-pool-preview).
        pub fips_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the GPU MIG instance profile for supported GPU VM SKU. The allowed values are `MIG1g`, `MIG2g`, `MIG3g`, `MIG4g` and `MIG7g`. Changing this forces a new resource to be created.
        pub gpu_instance: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the nodes in this Node Pool have host encryption enabled? Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Additional fields must be configured depending on the value of this field - see below.
        pub host_encryption_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The fully qualified resource ID of the Dedicated Host Group to provision virtual machines from. Changing this forces a new resource to be created.
        pub host_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `kubelet_config` block as defined below. Changing this forces a new resource to be created.
        pub kubelet_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolKubeletConfig,
            >,
        >,
        /// The type of disk used by kubelet. Possible values are `OS` and `Temporary`.
        pub kubelet_disk_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Kubernetes Cluster where this Node Pool should exist. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The type of Default Node Pool for the Kubernetes Cluster must be `VirtualMachineScaleSets` to attach multiple node pools.
        pub kubernetes_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// A `linux_os_config` block as defined below. Changing this forces a new resource to be created.
        pub linux_os_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolLinuxOsConfig,
            >,
        >,
        pub max_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum number of pods that can run on each agent. Changing this forces a new resource to be created.
        pub max_pods: pulumi_gestalt_rust::Output<i32>,
        pub min_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Should this Node Pool be used for System or User resources? Possible values are `System` and `User`. Defaults to `User`.
        pub mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Node Pool which should be created within the Kubernetes Cluster. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A Windows Node Pool cannot have a `name` longer than 6 characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub node_count: pulumi_gestalt_rust::Output<i32>,
        /// A map of Kubernetes labels which should be applied to nodes in this Node Pool.
        pub node_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `node_network_profile` block as documented below.
        pub node_network_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolNodeNetworkProfile,
            >,
        >,
        /// Should each node have a Public IP Address? Changing this forces a new resource to be created.
        pub node_public_ip_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Resource ID for the Public IP Addresses Prefix for the nodes in this Node Pool. `node_public_ip_enabled` should be `true`. Changing this forces a new resource to be created.
        pub node_public_ip_prefix_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of Kubernetes taints which should be applied to nodes in the agent pool (e.g `key=value:NoSchedule`).
        pub node_taints: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Version of Kubernetes used for the Agents. If not specified, the latest recommended version will be used at provisioning time (but won't auto-upgrade). AKS does not require an exact patch version to be specified, minor version aliases such as `1.22` are also supported. - The minor version's latest GA patch is automatically chosen in that case. More details can be found in [the documentation](https://docs.microsoft.com/en-us/azure/aks/supported-kubernetes-versions?tabs=azure-cli#alias-minor-version).
        ///
        /// > **Note:** This version must be supported by the Kubernetes Cluster - as such the version of Kubernetes used on the Cluster/Control Plane may need to be upgraded first.
        pub orchestrator_version: pulumi_gestalt_rust::Output<String>,
        /// The Agent Operating System disk size in GB. Changing this forces a new resource to be created.
        pub os_disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// The type of disk which should be used for the Operating System. Possible values are `Ephemeral` and `Managed`. Defaults to `Managed`. Changing this forces a new resource to be created.
        pub os_disk_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the OS SKU used by the agent pool. Possible values are `AzureLinux`, `Ubuntu`, `Windows2019` and `Windows2022`. If not specified, the default is `Ubuntu` if OSType=Linux or `Windows2019` if OSType=Windows. And the default Windows OSSKU will be changed to `Windows2022` after Windows2019 is deprecated. Changing this from `AzureLinux` or `Ubuntu` to `AzureLinux` or `Ubuntu` will not replace the resource, otherwise it forces a new resource to be created.
        pub os_sku: pulumi_gestalt_rust::Output<String>,
        /// The Operating System which should be used for this Node Pool. Changing this forces a new resource to be created. Possible values are `Linux` and `Windows`. Defaults to `Linux`.
        pub os_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Subnet where the pods in the Node Pool should exist. Changing this forces a new resource to be created.
        pub pod_subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Priority for Virtual Machines within the Virtual Machine Scale Set that powers this Node Pool. Possible values are `Regular` and `Spot`. Defaults to `Regular`. Changing this forces a new resource to be created.
        pub priority: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Proximity Placement Group where the Virtual Machine Scale Set that powers this Node Pool will be placed. Changing this forces a new resource to be created.
        ///
        /// > **Note:** When setting `priority` to Spot - you must configure an `eviction_policy`, `spot_max_price` and add the applicable `node_labels` and `node_taints` [as per the Azure Documentation](https://docs.microsoft.com/azure/aks/spot-node-pool).
        pub proximity_placement_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies how the node pool should deal with scaled-down nodes. Allowed values are `Delete` and `Deallocate`. Defaults to `Delete`.
        pub scale_down_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Snapshot which should be used to create this Node Pool. Changing this forces a new resource to be created.
        pub snapshot_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum price you're willing to pay in USD per Virtual Machine. Valid values are `-1` (the current on-demand price for a Virtual Machine) or a positive value with up to five decimal places. Changing this forces a new resource to be created.
        ///
        /// > **Note:** This field can only be configured when `priority` is set to `Spot`.
        pub spot_max_price: pulumi_gestalt_rust::Output<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > At this time there's a bug in the AKS API where Tags for a Node Pool are not stored in the correct case - you may wish to use [`ignoreChanges`](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) functionality to ignore changes to the casing until this is fixed in the AKS API.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Used to specify whether the UltraSSD is enabled in the Node Pool. Defaults to `false`. See [the documentation](https://docs.microsoft.com/azure/aks/use-ultra-disks) for more information. Changing this forces a new resource to be created.
        pub ultra_ssd_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `upgrade_settings` block as documented below.
        pub upgrade_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolUpgradeSettings,
            >,
        >,
        /// The SKU which should be used for the Virtual Machines used in this Node Pool. Changing this forces a new resource to be created.
        pub vm_size: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet where this Node Pool should exist. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A route table must be configured on this Subnet.
        pub vnet_subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `windows_profile` block as documented below. Changing this forces a new resource to be created.
        pub windows_profile: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesClusterNodePoolWindowsProfile,
            >,
        >,
        /// Used to specify the workload runtime. Allowed values are `OCIContainer` and `WasmWasi`.
        ///
        /// > **Note:** WebAssembly System Interface node pools are in Public Preview - more information and details on how to opt into the preview can be found in [this article](https://docs.microsoft.com/azure/aks/use-wasi-node-pools)
        pub workload_runtime: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a list of Availability Zones in which this Kubernetes Cluster Node Pool should be located. Changing this forces a new Kubernetes Cluster Node Pool to be created.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KubernetesClusterNodePoolArgs,
    ) -> KubernetesClusterNodePoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_scaling_enabled_binding_1 = args
            .auto_scaling_enabled
            .get_output(context);
        let auto_scaling_enabled_binding = auto_scaling_enabled_binding_1.get_inner();
        let capacity_reservation_group_id_binding_1 = args
            .capacity_reservation_group_id
            .get_output(context);
        let capacity_reservation_group_id_binding = capacity_reservation_group_id_binding_1
            .get_inner();
        let eviction_policy_binding_1 = args.eviction_policy.get_output(context);
        let eviction_policy_binding = eviction_policy_binding_1.get_inner();
        let fips_enabled_binding_1 = args.fips_enabled.get_output(context);
        let fips_enabled_binding = fips_enabled_binding_1.get_inner();
        let gpu_instance_binding_1 = args.gpu_instance.get_output(context);
        let gpu_instance_binding = gpu_instance_binding_1.get_inner();
        let host_encryption_enabled_binding_1 = args
            .host_encryption_enabled
            .get_output(context);
        let host_encryption_enabled_binding = host_encryption_enabled_binding_1
            .get_inner();
        let host_group_id_binding_1 = args.host_group_id.get_output(context);
        let host_group_id_binding = host_group_id_binding_1.get_inner();
        let kubelet_config_binding_1 = args.kubelet_config.get_output(context);
        let kubelet_config_binding = kubelet_config_binding_1.get_inner();
        let kubelet_disk_type_binding_1 = args.kubelet_disk_type.get_output(context);
        let kubelet_disk_type_binding = kubelet_disk_type_binding_1.get_inner();
        let kubernetes_cluster_id_binding_1 = args
            .kubernetes_cluster_id
            .get_output(context);
        let kubernetes_cluster_id_binding = kubernetes_cluster_id_binding_1.get_inner();
        let linux_os_config_binding_1 = args.linux_os_config.get_output(context);
        let linux_os_config_binding = linux_os_config_binding_1.get_inner();
        let max_count_binding_1 = args.max_count.get_output(context);
        let max_count_binding = max_count_binding_1.get_inner();
        let max_pods_binding_1 = args.max_pods.get_output(context);
        let max_pods_binding = max_pods_binding_1.get_inner();
        let min_count_binding_1 = args.min_count.get_output(context);
        let min_count_binding = min_count_binding_1.get_inner();
        let mode_binding_1 = args.mode.get_output(context);
        let mode_binding = mode_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let node_count_binding_1 = args.node_count.get_output(context);
        let node_count_binding = node_count_binding_1.get_inner();
        let node_labels_binding_1 = args.node_labels.get_output(context);
        let node_labels_binding = node_labels_binding_1.get_inner();
        let node_network_profile_binding_1 = args
            .node_network_profile
            .get_output(context);
        let node_network_profile_binding = node_network_profile_binding_1.get_inner();
        let node_public_ip_enabled_binding_1 = args
            .node_public_ip_enabled
            .get_output(context);
        let node_public_ip_enabled_binding = node_public_ip_enabled_binding_1
            .get_inner();
        let node_public_ip_prefix_id_binding_1 = args
            .node_public_ip_prefix_id
            .get_output(context);
        let node_public_ip_prefix_id_binding = node_public_ip_prefix_id_binding_1
            .get_inner();
        let node_taints_binding_1 = args.node_taints.get_output(context);
        let node_taints_binding = node_taints_binding_1.get_inner();
        let orchestrator_version_binding_1 = args
            .orchestrator_version
            .get_output(context);
        let orchestrator_version_binding = orchestrator_version_binding_1.get_inner();
        let os_disk_size_gb_binding_1 = args.os_disk_size_gb.get_output(context);
        let os_disk_size_gb_binding = os_disk_size_gb_binding_1.get_inner();
        let os_disk_type_binding_1 = args.os_disk_type.get_output(context);
        let os_disk_type_binding = os_disk_type_binding_1.get_inner();
        let os_sku_binding_1 = args.os_sku.get_output(context);
        let os_sku_binding = os_sku_binding_1.get_inner();
        let os_type_binding_1 = args.os_type.get_output(context);
        let os_type_binding = os_type_binding_1.get_inner();
        let pod_subnet_id_binding_1 = args.pod_subnet_id.get_output(context);
        let pod_subnet_id_binding = pod_subnet_id_binding_1.get_inner();
        let priority_binding_1 = args.priority.get_output(context);
        let priority_binding = priority_binding_1.get_inner();
        let proximity_placement_group_id_binding_1 = args
            .proximity_placement_group_id
            .get_output(context);
        let proximity_placement_group_id_binding = proximity_placement_group_id_binding_1
            .get_inner();
        let scale_down_mode_binding_1 = args.scale_down_mode.get_output(context);
        let scale_down_mode_binding = scale_down_mode_binding_1.get_inner();
        let snapshot_id_binding_1 = args.snapshot_id.get_output(context);
        let snapshot_id_binding = snapshot_id_binding_1.get_inner();
        let spot_max_price_binding_1 = args.spot_max_price.get_output(context);
        let spot_max_price_binding = spot_max_price_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let ultra_ssd_enabled_binding_1 = args.ultra_ssd_enabled.get_output(context);
        let ultra_ssd_enabled_binding = ultra_ssd_enabled_binding_1.get_inner();
        let upgrade_settings_binding_1 = args.upgrade_settings.get_output(context);
        let upgrade_settings_binding = upgrade_settings_binding_1.get_inner();
        let vm_size_binding_1 = args.vm_size.get_output(context);
        let vm_size_binding = vm_size_binding_1.get_inner();
        let vnet_subnet_id_binding_1 = args.vnet_subnet_id.get_output(context);
        let vnet_subnet_id_binding = vnet_subnet_id_binding_1.get_inner();
        let windows_profile_binding_1 = args.windows_profile.get_output(context);
        let windows_profile_binding = windows_profile_binding_1.get_inner();
        let workload_runtime_binding_1 = args.workload_runtime.get_output(context);
        let workload_runtime_binding = workload_runtime_binding_1.get_inner();
        let zones_binding_1 = args.zones.get_output(context);
        let zones_binding = zones_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/kubernetesClusterNodePool:KubernetesClusterNodePool"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoScalingEnabled".into(),
                    value: &auto_scaling_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "capacityReservationGroupId".into(),
                    value: &capacity_reservation_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "evictionPolicy".into(),
                    value: &eviction_policy_binding,
                },
                register_interface::ObjectField {
                    name: "fipsEnabled".into(),
                    value: &fips_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "gpuInstance".into(),
                    value: &gpu_instance_binding,
                },
                register_interface::ObjectField {
                    name: "hostEncryptionEnabled".into(),
                    value: &host_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "hostGroupId".into(),
                    value: &host_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "kubeletConfig".into(),
                    value: &kubelet_config_binding,
                },
                register_interface::ObjectField {
                    name: "kubeletDiskType".into(),
                    value: &kubelet_disk_type_binding,
                },
                register_interface::ObjectField {
                    name: "kubernetesClusterId".into(),
                    value: &kubernetes_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "linuxOsConfig".into(),
                    value: &linux_os_config_binding,
                },
                register_interface::ObjectField {
                    name: "maxCount".into(),
                    value: &max_count_binding,
                },
                register_interface::ObjectField {
                    name: "maxPods".into(),
                    value: &max_pods_binding,
                },
                register_interface::ObjectField {
                    name: "minCount".into(),
                    value: &min_count_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeCount".into(),
                    value: &node_count_binding,
                },
                register_interface::ObjectField {
                    name: "nodeLabels".into(),
                    value: &node_labels_binding,
                },
                register_interface::ObjectField {
                    name: "nodeNetworkProfile".into(),
                    value: &node_network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "nodePublicIpEnabled".into(),
                    value: &node_public_ip_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "nodePublicIpPrefixId".into(),
                    value: &node_public_ip_prefix_id_binding,
                },
                register_interface::ObjectField {
                    name: "nodeTaints".into(),
                    value: &node_taints_binding,
                },
                register_interface::ObjectField {
                    name: "orchestratorVersion".into(),
                    value: &orchestrator_version_binding,
                },
                register_interface::ObjectField {
                    name: "osDiskSizeGb".into(),
                    value: &os_disk_size_gb_binding,
                },
                register_interface::ObjectField {
                    name: "osDiskType".into(),
                    value: &os_disk_type_binding,
                },
                register_interface::ObjectField {
                    name: "osSku".into(),
                    value: &os_sku_binding,
                },
                register_interface::ObjectField {
                    name: "osType".into(),
                    value: &os_type_binding,
                },
                register_interface::ObjectField {
                    name: "podSubnetId".into(),
                    value: &pod_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "proximityPlacementGroupId".into(),
                    value: &proximity_placement_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "scaleDownMode".into(),
                    value: &scale_down_mode_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotId".into(),
                    value: &snapshot_id_binding,
                },
                register_interface::ObjectField {
                    name: "spotMaxPrice".into(),
                    value: &spot_max_price_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "ultraSsdEnabled".into(),
                    value: &ultra_ssd_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "upgradeSettings".into(),
                    value: &upgrade_settings_binding,
                },
                register_interface::ObjectField {
                    name: "vmSize".into(),
                    value: &vm_size_binding,
                },
                register_interface::ObjectField {
                    name: "vnetSubnetId".into(),
                    value: &vnet_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "windowsProfile".into(),
                    value: &windows_profile_binding,
                },
                register_interface::ObjectField {
                    name: "workloadRuntime".into(),
                    value: &workload_runtime_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KubernetesClusterNodePoolResult {
            auto_scaling_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoScalingEnabled"),
            ),
            capacity_reservation_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacityReservationGroupId"),
            ),
            eviction_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("evictionPolicy"),
            ),
            fips_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fipsEnabled"),
            ),
            gpu_instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gpuInstance"),
            ),
            host_encryption_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostEncryptionEnabled"),
            ),
            host_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostGroupId"),
            ),
            kubelet_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubeletConfig"),
            ),
            kubelet_disk_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubeletDiskType"),
            ),
            kubernetes_cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubernetesClusterId"),
            ),
            linux_os_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linuxOsConfig"),
            ),
            max_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxCount"),
            ),
            max_pods: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxPods"),
            ),
            min_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minCount"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            node_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeCount"),
            ),
            node_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeLabels"),
            ),
            node_network_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeNetworkProfile"),
            ),
            node_public_ip_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodePublicIpEnabled"),
            ),
            node_public_ip_prefix_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodePublicIpPrefixId"),
            ),
            node_taints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeTaints"),
            ),
            orchestrator_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("orchestratorVersion"),
            ),
            os_disk_size_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osDiskSizeGb"),
            ),
            os_disk_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osDiskType"),
            ),
            os_sku: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osSku"),
            ),
            os_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osType"),
            ),
            pod_subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("podSubnetId"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            proximity_placement_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("proximityPlacementGroupId"),
            ),
            scale_down_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scaleDownMode"),
            ),
            snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotId"),
            ),
            spot_max_price: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("spotMaxPrice"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            ultra_ssd_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ultraSsdEnabled"),
            ),
            upgrade_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("upgradeSettings"),
            ),
            vm_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmSize"),
            ),
            vnet_subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vnetSubnetId"),
            ),
            windows_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("windowsProfile"),
            ),
            workload_runtime: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadRuntime"),
            ),
            zones: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
