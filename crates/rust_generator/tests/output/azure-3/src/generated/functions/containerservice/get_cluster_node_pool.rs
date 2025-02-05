pub mod get_cluster_node_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterNodePoolArgs {
        /// The Name of the Kubernetes Cluster where this Node Pool is located.
        #[builder(into)]
        pub kubernetes_cluster_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of this Kubernetes Cluster Node Pool.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Kubernetes Cluster exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetClusterNodePoolResult {
        /// Does this Node Pool have Auto-Scaling enabled?
        pub auto_scaling_enabled: pulumi_wasm_rust::Output<bool>,
        /// The eviction policy used for Virtual Machines in the Virtual Machine Scale Set, when `priority` is set to `Spot`.
        pub eviction_policy: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kubernetes_cluster_name: pulumi_wasm_rust::Output<String>,
        /// The maximum number of Nodes allowed when auto-scaling is enabled.
        pub max_count: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of Pods allowed on each Node in this Node Pool.
        pub max_pods: pulumi_wasm_rust::Output<i32>,
        /// The minimum number of Nodes allowed when auto-scaling is enabled.
        pub min_count: pulumi_wasm_rust::Output<i32>,
        /// The Mode for this Node Pool, specifying how these Nodes should be used (for either System or User resources).
        pub mode: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The current number of Nodes in the Node Pool.
        pub node_count: pulumi_wasm_rust::Output<i32>,
        /// A map of Kubernetes Labels applied to each Node in this Node Pool.
        pub node_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Do nodes in this Node Pool have a Public IP Address?
        pub node_public_ip_enabled: pulumi_wasm_rust::Output<bool>,
        /// Resource ID for the Public IP Addresses Prefix for the nodes in this Agent Pool.
        pub node_public_ip_prefix_id: pulumi_wasm_rust::Output<String>,
        /// A map of Kubernetes Taints applied to each Node in this Node Pool.
        pub node_taints: pulumi_wasm_rust::Output<Vec<String>>,
        /// The version of Kubernetes configured on each Node in this Node Pool.
        pub orchestrator_version: pulumi_wasm_rust::Output<String>,
        /// The size of the OS Disk on each Node in this Node Pool.
        pub os_disk_size_gb: pulumi_wasm_rust::Output<i32>,
        /// The type of the OS Disk on each Node in this Node Pool.
        pub os_disk_type: pulumi_wasm_rust::Output<String>,
        /// The operating system used on each Node in this Node Pool.
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The priority of the Virtual Machines in the Virtual Machine Scale Set backing this Node Pool.
        pub priority: pulumi_wasm_rust::Output<String>,
        /// The ID of the Proximity Placement Group where the Virtual Machine Scale Set backing this Node Pool will be placed.
        pub proximity_placement_group_id: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The maximum price being paid for Virtual Machines in this Scale Set. `-1` means the current on-demand price for a Virtual Machine.
        pub spot_max_price: pulumi_wasm_rust::Output<f64>,
        /// A mapping of tags assigned to the Kubernetes Cluster Node Pool.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A `upgrade_settings` block as documented below.
        pub upgrade_settings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetClusterNodePoolUpgradeSetting,
            >,
        >,
        /// The size of the Virtual Machines used in the Virtual Machine Scale Set backing this Node Pool.
        pub vm_size: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet in which this Node Pool exists.
        pub vnet_subnet_id: pulumi_wasm_rust::Output<String>,
        /// A list of the Availability Zones where the Nodes in this Node Pool exist.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClusterNodePoolArgs,
    ) -> GetClusterNodePoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kubernetes_cluster_name_binding = args
            .kubernetes_cluster_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerservice/getClusterNodePool:getClusterNodePool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kubernetesClusterName".into(),
                    value: &kubernetes_cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterNodePoolResult {
            auto_scaling_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoScalingEnabled"),
            ),
            eviction_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("evictionPolicy"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kubernetes_cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kubernetesClusterName"),
            ),
            max_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxCount"),
            ),
            max_pods: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxPods"),
            ),
            min_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minCount"),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(o.extract_field("mode")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            node_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeCount"),
            ),
            node_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeLabels"),
            ),
            node_public_ip_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodePublicIpEnabled"),
            ),
            node_public_ip_prefix_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodePublicIpPrefixId"),
            ),
            node_taints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeTaints"),
            ),
            orchestrator_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("orchestratorVersion"),
            ),
            os_disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("osDiskSizeGb"),
            ),
            os_disk_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("osDiskType"),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(o.extract_field("osType")),
            priority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            proximity_placement_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("proximityPlacementGroupId"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            spot_max_price: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("spotMaxPrice"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            upgrade_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("upgradeSettings"),
            ),
            vm_size: pulumi_wasm_rust::__private::into_domain(o.extract_field("vmSize")),
            vnet_subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vnetSubnetId"),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
