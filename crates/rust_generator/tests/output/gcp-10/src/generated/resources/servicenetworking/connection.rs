/// Manages a private VPC connection with a GCP service provider. For more information see
/// [the official documentation](https://cloud.google.com/vpc/docs/configure-private-services-access#creating-connection)
/// and
/// [API](https://cloud.google.com/service-infrastructure/docs/service-networking/reference/rest/v1/services.connections).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = connection::create(
///         "default",
///         ConnectionArgs::builder()
///             .network("${peeringNetwork.id}")
///             .reserved_peering_ranges(vec!["${privateIpAlloc.name}",])
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
///     let peeringNetwork = network::create(
///         "peeringNetwork",
///         NetworkArgs::builder().name("peering-network").build_struct(),
///     );
///     let peeringRoutes = network_peering_routes_config::create(
///         "peeringRoutes",
///         NetworkPeeringRoutesConfigArgs::builder()
///             .export_custom_routes(true)
///             .import_custom_routes(true)
///             .network("${peeringNetwork.name}")
///             .peering("${default.peering}")
///             .build_struct(),
///     );
///     let privateIpAlloc = global_address::create(
///         "privateIpAlloc",
///         GlobalAddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("private-ip-alloc")
///             .network("${peeringNetwork.id}")
///             .prefix_length(16)
///             .purpose("VPC_PEERING")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ServiceNetworkingConnection can be imported using any of these accepted formats
///
/// * `{{peering-network}}:{{service}}`
///
/// * `projects/{{project}}/global/networks/{{peering-network}}:{{service}}`
///
/// When using the `pulumi import` command, NAME_HERE can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:servicenetworking/connection:Connection default {{peering-network}}:{{service}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:servicenetworking/connection:Connection default /projects/{{project}}/global/networks/{{peering-network}}:{{service}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of VPC network connected with service producers using VPC peering.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Named IP address range(s) of PEERING type reserved for
        /// this service provider. Note that invoking this method with a different range when connection
        /// is already established will not reallocate already provisioned service producer subnetworks.
        #[builder(into)]
        pub reserved_peering_ranges: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Provider peering service that is managing peering connectivity for a
        /// service provider organization. For Google services that support this functionality it is
        /// 'servicenetworking.googleapis.com'.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When set to true, enforce an update of the reserved peering ranges on the existing service networking connection in case of a new connection creation failure.
        #[builder(into, default)]
        pub update_on_creation_fail: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of VPC network connected with service producers using VPC peering.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The name of the VPC Network Peering connection that was created by the service producer.
        pub peering: pulumi_gestalt_rust::Output<String>,
        /// Named IP address range(s) of PEERING type reserved for
        /// this service provider. Note that invoking this method with a different range when connection
        /// is already established will not reallocate already provisioned service producer subnetworks.
        pub reserved_peering_ranges: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Provider peering service that is managing peering connectivity for a
        /// service provider organization. For Google services that support this functionality it is
        /// 'servicenetworking.googleapis.com'.
        pub service: pulumi_gestalt_rust::Output<String>,
        /// When set to true, enforce an update of the reserved peering ranges on the existing service networking connection in case of a new connection creation failure.
        pub update_on_creation_fail: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_policy_binding = args.deletion_policy.get_output(context);
        let network_binding = args.network.get_output(context);
        let reserved_peering_ranges_binding = args
            .reserved_peering_ranges
            .get_output(context);
        let service_binding = args.service.get_output(context);
        let update_on_creation_fail_binding = args
            .update_on_creation_fail
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:servicenetworking/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionPolicy".into(),
                    value: deletion_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservedPeeringRanges".into(),
                    value: reserved_peering_ranges_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "updateOnCreationFail".into(),
                    value: update_on_creation_fail_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionResult {
            deletion_policy: o.get_field("deletionPolicy"),
            network: o.get_field("network"),
            peering: o.get_field("peering"),
            reserved_peering_ranges: o.get_field("reservedPeeringRanges"),
            service: o.get_field("service"),
            update_on_creation_fail: o.get_field("updateOnCreationFail"),
        }
    }
}
