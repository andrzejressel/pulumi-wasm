#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cluster_node_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterNodePoolArgs {
        /// The Name of the Kubernetes Cluster where this Node Pool is located.
        #[builder(into)]
        pub kubernetes_cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Kubernetes Cluster Node Pool.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Kubernetes Cluster exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetClusterNodePoolResult {
        /// Does this Node Pool have Auto-Scaling enabled?
        pub auto_scaling_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The eviction policy used for Virtual Machines in the Virtual Machine Scale Set, when `priority` is set to `Spot`.
        pub eviction_policy: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kubernetes_cluster_name: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of Nodes allowed when auto-scaling is enabled.
        pub max_count: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of Pods allowed on each Node in this Node Pool.
        pub max_pods: pulumi_gestalt_rust::Output<i32>,
        /// The minimum number of Nodes allowed when auto-scaling is enabled.
        pub min_count: pulumi_gestalt_rust::Output<i32>,
        /// The Mode for this Node Pool, specifying how these Nodes should be used (for either System or User resources).
        pub mode: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The current number of Nodes in the Node Pool.
        pub node_count: pulumi_gestalt_rust::Output<i32>,
        /// A map of Kubernetes Labels applied to each Node in this Node Pool.
        pub node_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Do nodes in this Node Pool have a Public IP Address?
        pub node_public_ip_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Resource ID for the Public IP Addresses Prefix for the nodes in this Agent Pool.
        pub node_public_ip_prefix_id: pulumi_gestalt_rust::Output<String>,
        /// A map of Kubernetes Taints applied to each Node in this Node Pool.
        pub node_taints: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The version of Kubernetes configured on each Node in this Node Pool.
        pub orchestrator_version: pulumi_gestalt_rust::Output<String>,
        /// The size of the OS Disk on each Node in this Node Pool.
        pub os_disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// The type of the OS Disk on each Node in this Node Pool.
        pub os_disk_type: pulumi_gestalt_rust::Output<String>,
        /// The operating system used on each Node in this Node Pool.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// The priority of the Virtual Machines in the Virtual Machine Scale Set backing this Node Pool.
        pub priority: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Proximity Placement Group where the Virtual Machine Scale Set backing this Node Pool will be placed.
        pub proximity_placement_group_id: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The maximum price being paid for Virtual Machines in this Scale Set. `-1` means the current on-demand price for a Virtual Machine.
        pub spot_max_price: pulumi_gestalt_rust::Output<f64>,
        /// A mapping of tags assigned to the Kubernetes Cluster Node Pool.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A `upgrade_settings` block as documented below.
        pub upgrade_settings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetClusterNodePoolUpgradeSetting,
            >,
        >,
        /// The size of the Virtual Machines used in the Virtual Machine Scale Set backing this Node Pool.
        pub vm_size: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet in which this Node Pool exists.
        pub vnet_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A list of the Availability Zones where the Nodes in this Node Pool exist.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetClusterNodePoolArgs,
    ) -> GetClusterNodePoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let kubernetes_cluster_name_binding = args
            .kubernetes_cluster_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerservice/getClusterNodePool:getClusterNodePool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubernetesClusterName".into(),
                    value: kubernetes_cluster_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClusterNodePoolResult {
            auto_scaling_enabled: o.get_field("autoScalingEnabled"),
            eviction_policy: o.get_field("evictionPolicy"),
            id: o.get_field("id"),
            kubernetes_cluster_name: o.get_field("kubernetesClusterName"),
            max_count: o.get_field("maxCount"),
            max_pods: o.get_field("maxPods"),
            min_count: o.get_field("minCount"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            node_count: o.get_field("nodeCount"),
            node_labels: o.get_field("nodeLabels"),
            node_public_ip_enabled: o.get_field("nodePublicIpEnabled"),
            node_public_ip_prefix_id: o.get_field("nodePublicIpPrefixId"),
            node_taints: o.get_field("nodeTaints"),
            orchestrator_version: o.get_field("orchestratorVersion"),
            os_disk_size_gb: o.get_field("osDiskSizeGb"),
            os_disk_type: o.get_field("osDiskType"),
            os_type: o.get_field("osType"),
            priority: o.get_field("priority"),
            proximity_placement_group_id: o.get_field("proximityPlacementGroupId"),
            resource_group_name: o.get_field("resourceGroupName"),
            spot_max_price: o.get_field("spotMaxPrice"),
            tags: o.get_field("tags"),
            upgrade_settings: o.get_field("upgradeSettings"),
            vm_size: o.get_field("vmSize"),
            vnet_subnet_id: o.get_field("vnetSubnetId"),
            zones: o.get_field("zones"),
        }
    }
}
