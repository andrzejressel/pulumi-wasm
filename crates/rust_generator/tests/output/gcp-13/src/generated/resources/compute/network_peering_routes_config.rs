/// Manage a network peering's route settings without managing the peering as
/// a whole. This resource is primarily intended for use with GCP-generated
/// peerings that shouldn't otherwise be managed by other tools. Deleting this
/// resource is a no-op and the peering will not be modified.
///
///
/// To get more information about NetworkPeeringRoutesConfig, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/networks/updatePeering)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vpc/docs/vpc-peering)
///
/// ## Example Usage
///
/// ### Network Peering Routes Config Basic
///
///
/// ```yaml
/// resources:
///   peeringPrimaryRoutes:
///     type: gcp:compute:NetworkPeeringRoutesConfig
///     name: peering_primary_routes
///     properties:
///       peering: ${peeringPrimary.name}
///       network: ${networkPrimary.name}
///       importCustomRoutes: true
///       exportCustomRoutes: true
///   peeringPrimary:
///     type: gcp:compute:NetworkPeering
///     name: peering_primary
///     properties:
///       name: primary-peering
///       network: ${networkPrimary.id}
///       peerNetwork: ${networkSecondary.id}
///       importCustomRoutes: true
///       exportCustomRoutes: true
///   peeringSecondary:
///     type: gcp:compute:NetworkPeering
///     name: peering_secondary
///     properties:
///       name: secondary-peering
///       network: ${networkSecondary.id}
///       peerNetwork: ${networkPrimary.id}
///   networkPrimary:
///     type: gcp:compute:Network
///     name: network_primary
///     properties:
///       name: primary-network
///       autoCreateSubnetworks: 'false'
///   networkSecondary:
///     type: gcp:compute:Network
///     name: network_secondary
///     properties:
///       name: secondary-network
///       autoCreateSubnetworks: 'false'
/// ```
/// ### Network Peering Routes Config Gke
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let containerNetwork = network::create(
///         "containerNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("container-network")
///             .build_struct(),
///     );
///     let containerSubnetwork = subnetwork::create(
///         "containerSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.36.0/24")
///             .name("container-subnetwork")
///             .network("${containerNetwork.name}")
///             .private_ip_google_access(true)
///             .region("us-central1")
///             .secondary_ip_ranges(
///                 vec![
///                     SubnetworkSecondaryIpRange::builder().ipCidrRange("10.0.0.0/19")
///                     .rangeName("pod").build_struct(),
///                     SubnetworkSecondaryIpRange::builder().ipCidrRange("10.0.32.0/22")
///                     .rangeName("svc").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let peeringGkeRoutes = network_peering_routes_config::create(
///         "peeringGkeRoutes",
///         NetworkPeeringRoutesConfigArgs::builder()
///             .export_custom_routes(true)
///             .import_custom_routes(true)
///             .network("${containerNetwork.name}")
///             .peering("${privateCluster.privateClusterConfig.peeringName}")
///             .build_struct(),
///     );
///     let privateCluster = cluster::create(
///         "privateCluster",
///         ClusterArgs::builder()
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .ip_allocation_policy(
///                 ClusterIpAllocationPolicy::builder()
///                     .clusterSecondaryRangeName(
///                         "${containerSubnetwork.secondaryIpRanges[0].rangeName}",
///                     )
///                     .servicesSecondaryRangeName(
///                         "${containerSubnetwork.secondaryIpRanges[1].rangeName}",
///                     )
///                     .build_struct(),
///             )
///             .location("us-central1-a")
///             .master_authorized_networks_config(
///                 ClusterMasterAuthorizedNetworksConfig::builder().build_struct(),
///             )
///             .name("private-cluster")
///             .network("${containerNetwork.name}")
///             .private_cluster_config(
///                 ClusterPrivateClusterConfig::builder()
///                     .enablePrivateEndpoint(true)
///                     .enablePrivateNodes(true)
///                     .masterIpv4CidrBlock("10.42.0.0/28")
///                     .build_struct(),
///             )
///             .subnetwork("${containerSubnetwork.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// NetworkPeeringRoutesConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/networks/{{network}}/networkPeerings/{{peering}}`
///
/// * `{{project}}/{{network}}/{{peering}}`
///
/// * `{{network}}/{{peering}}`
///
/// When using the `pulumi import` command, NetworkPeeringRoutesConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkPeeringRoutesConfig:NetworkPeeringRoutesConfig default projects/{{project}}/global/networks/{{network}}/networkPeerings/{{peering}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkPeeringRoutesConfig:NetworkPeeringRoutesConfig default {{project}}/{{network}}/{{peering}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkPeeringRoutesConfig:NetworkPeeringRoutesConfig default {{network}}/{{peering}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_peering_routes_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkPeeringRoutesConfigArgs {
        /// Whether to export the custom routes to the peer network.
        #[builder(into)]
        pub export_custom_routes: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Whether to import the custom routes to the peer network.
        #[builder(into)]
        pub import_custom_routes: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The name of the primary network for the peering.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the peering.
        #[builder(into)]
        pub peering: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkPeeringRoutesConfigResult {
        /// Whether to export the custom routes to the peer network.
        pub export_custom_routes: pulumi_gestalt_rust::Output<bool>,
        /// Whether to import the custom routes to the peer network.
        pub import_custom_routes: pulumi_gestalt_rust::Output<bool>,
        /// The name of the primary network for the peering.
        ///
        ///
        /// - - -
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Name of the peering.
        pub peering: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkPeeringRoutesConfigArgs,
    ) -> NetworkPeeringRoutesConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let export_custom_routes_binding = args.export_custom_routes.get_output(context);
        let import_custom_routes_binding = args.import_custom_routes.get_output(context);
        let network_binding = args.network.get_output(context);
        let peering_binding = args.peering.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/networkPeeringRoutesConfig:NetworkPeeringRoutesConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportCustomRoutes".into(),
                    value: export_custom_routes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "importCustomRoutes".into(),
                    value: import_custom_routes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peering".into(),
                    value: peering_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkPeeringRoutesConfigResult {
            export_custom_routes: o.get_field("exportCustomRoutes"),
            import_custom_routes: o.get_field("importCustomRoutes"),
            network: o.get_field("network"),
            peering: o.get_field("peering"),
            project: o.get_field("project"),
        }
    }
}
