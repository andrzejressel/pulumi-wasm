/// Manages an Azure VMware Solution Cluster.
///
/// ## Example Usage
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// The count of the Azure VMware Solution Cluster nodes.
        #[builder(into)]
        pub cluster_node_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The name which should be used for this Azure VMware Solution Cluster. Changing this forces a new Azure VMware Solution Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Cluster SKU to use. Possible values are `av20`, `av36`, `av36t`, `av36p`, `av36pt`, `av52`, `av52t`, and `av64`. Changing this forces a new Azure VMware Solution Cluster to be created.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Azure VMware Solution Private Cloud in which to create this Cluster. Changing this forces a new Azure VMware Solution Cluster to be created.
        #[builder(into)]
        pub vmware_cloud_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The count of the Azure VMware Solution Cluster nodes.
        pub cluster_node_count: pulumi_gestalt_rust::Output<i32>,
        /// A number that identifies this Cluster in its Azure VMware Solution Private Cloud.
        pub cluster_number: pulumi_gestalt_rust::Output<i32>,
        /// A list of hosts in the Azure VMware Solution Cluster.
        pub hosts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name which should be used for this Azure VMware Solution Cluster. Changing this forces a new Azure VMware Solution Cluster to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Cluster SKU to use. Possible values are `av20`, `av36`, `av36t`, `av36p`, `av36pt`, `av52`, `av52t`, and `av64`. Changing this forces a new Azure VMware Solution Cluster to be created.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Azure VMware Solution Private Cloud in which to create this Cluster. Changing this forces a new Azure VMware Solution Cluster to be created.
        pub vmware_cloud_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_node_count_binding = args.cluster_node_count.get_output(context);
        let name_binding = args.name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let vmware_cloud_id_binding = args.vmware_cloud_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:avs/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterNodeCount".into(),
                    value: cluster_node_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmwareCloudId".into(),
                    value: vmware_cloud_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            cluster_node_count: o.get_field("clusterNodeCount"),
            cluster_number: o.get_field("clusterNumber"),
            hosts: o.get_field("hosts"),
            name: o.get_field("name"),
            sku_name: o.get_field("skuName"),
            vmware_cloud_id: o.get_field("vmwareCloudId"),
        }
    }
}
