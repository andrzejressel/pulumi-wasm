/// Manages an Azure VMware Solution Cluster.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleCluster = cluster::create(
///         "exampleCluster",
///         ClusterArgs::builder()
///             .cluster_node_count(3)
///             .name("example-Cluster")
///             .sku_name("av36")
///             .vmware_cloud_id("${examplePrivateCloud.id}")
///             .build_struct(),
///     );
///     let examplePrivateCloud = private_cloud::create(
///         "examplePrivateCloud",
///         PrivateCloudArgs::builder()
///             .internet_connection_enabled(false)
///             .location("${example.location}")
///             .management_cluster(
///                 PrivateCloudManagementCluster::builder().size(3).build_struct(),
///             )
///             .name("example-vmware-private-cloud")
///             .network_subnet_cidr("192.168.48.0/22")
///             .nsxt_password("QazWsx13$Edc")
///             .resource_group_name("${example.name}")
///             .sku_name("av36")
///             .vcenter_password("WsxEdc23$Rfv")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure VMware Solution Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:avs/cluster:Cluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AVS/privateClouds/privateCloud1/clusters/cluster1
/// ```
///
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// The count of the Azure VMware Solution Cluster nodes.
        #[builder(into)]
        pub cluster_node_count: pulumi_wasm_rust::Output<i32>,
        /// The name which should be used for this Azure VMware Solution Cluster. Changing this forces a new Azure VMware Solution Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Cluster SKU to use. Possible values are `av20`, `av36`, `av36t`, `av36p`, `av36pt`, `av52`, `av52t`, and `av64`. Changing this forces a new Azure VMware Solution Cluster to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure VMware Solution Private Cloud in which to create this Cluster. Changing this forces a new Azure VMware Solution Cluster to be created.
        #[builder(into)]
        pub vmware_cloud_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The count of the Azure VMware Solution Cluster nodes.
        pub cluster_node_count: pulumi_wasm_rust::Output<i32>,
        /// A number that identifies this Cluster in its Azure VMware Solution Private Cloud.
        pub cluster_number: pulumi_wasm_rust::Output<i32>,
        /// A list of hosts in the Azure VMware Solution Cluster.
        pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name which should be used for this Azure VMware Solution Cluster. Changing this forces a new Azure VMware Solution Cluster to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Cluster SKU to use. Possible values are `av20`, `av36`, `av36t`, `av36p`, `av36pt`, `av52`, `av52t`, and `av64`. Changing this forces a new Azure VMware Solution Cluster to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure VMware Solution Private Cloud in which to create this Cluster. Changing this forces a new Azure VMware Solution Cluster to be created.
        pub vmware_cloud_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_node_count_binding = args.cluster_node_count.get_inner();
        let name_binding = args.name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let vmware_cloud_id_binding = args.vmware_cloud_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:avs/cluster:Cluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterNodeCount".into(),
                    value: &cluster_node_count_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "vmwareCloudId".into(),
                    value: &vmware_cloud_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterNodeCount".into(),
                },
                register_interface::ResultField {
                    name: "clusterNumber".into(),
                },
                register_interface::ResultField {
                    name: "hosts".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "vmwareCloudId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            cluster_node_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterNodeCount").unwrap(),
            ),
            cluster_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterNumber").unwrap(),
            ),
            hosts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hosts").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            vmware_cloud_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareCloudId").unwrap(),
            ),
        }
    }
}
