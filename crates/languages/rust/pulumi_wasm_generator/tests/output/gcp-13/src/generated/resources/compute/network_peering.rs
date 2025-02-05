/// Manages a network peering within GCE. For more information see
/// [the official documentation](https://cloud.google.com/compute/docs/vpc/vpc-peering)
/// and
/// [API](https://cloud.google.com/compute/docs/reference/latest/networks).
///
/// > Both networks must create a peering with each other for the peering
/// to be functional.
///
/// > Subnets IP ranges across peered VPC networks cannot overlap.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   peering1:
///     type: gcp:compute:NetworkPeering
///     properties:
///       name: peering1
///       network: ${default.selfLink}
///       peerNetwork: ${other.selfLink}
///   peering2:
///     type: gcp:compute:NetworkPeering
///     properties:
///       name: peering2
///       network: ${other.selfLink}
///       peerNetwork: ${default.selfLink}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: foobar
///       autoCreateSubnetworks: 'false'
///   other:
///     type: gcp:compute:Network
///     properties:
///       name: other
///       autoCreateSubnetworks: 'false'
/// ```
///
/// ## Import
///
/// VPC network peerings can be imported using the name and project of the primary network the peering exists in and the name of the network peering
///
/// * `{{project_id}}/{{network_id}}/{{peering_id}}`
///
/// When using the `pulumi import` command, VPC network peerings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkPeering:NetworkPeering default {{project_id}}/{{network_id}}/{{peering_id}}
/// ```
///
pub mod network_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkPeeringArgs {
        /// Whether to export the custom routes to the peer network. Defaults to `false`.
        #[builder(into, default)]
        pub export_custom_routes: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether subnet routes with public IP range are exported. The default value is true, all subnet routes are exported. The IPv4 special-use ranges (https://en.wikipedia.org/wiki/IPv4#Special_addresses) are always exported to peers and are not controlled by this field.
        #[builder(into, default)]
        pub export_subnet_routes_with_public_ip: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to import the custom routes from the peer network. Defaults to `false`.
        #[builder(into, default)]
        pub import_custom_routes: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether subnet routes with public IP range are imported. The default value is false. The IPv4 special-use ranges (https://en.wikipedia.org/wiki/IPv4#Special_addresses) are always imported from peers and are not controlled by this field.
        #[builder(into, default)]
        pub import_subnet_routes_with_public_ip: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Name of the peering.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The primary network of the peering.
        #[builder(into)]
        pub network: pulumi_wasm_rust::InputOrOutput<String>,
        /// The peer network in the peering. The peer network
        /// may belong to a different project.
        #[builder(into)]
        pub peer_network: pulumi_wasm_rust::InputOrOutput<String>,
        /// Which IP version(s) of traffic and routes are allowed to be imported or exported between peer networks. The default value is IPV4_ONLY. Possible values: ["IPV4_ONLY", "IPV4_IPV6"].
        #[builder(into, default)]
        pub stack_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkPeeringResult {
        /// Whether to export the custom routes to the peer network. Defaults to `false`.
        pub export_custom_routes: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether subnet routes with public IP range are exported. The default value is true, all subnet routes are exported. The IPv4 special-use ranges (https://en.wikipedia.org/wiki/IPv4#Special_addresses) are always exported to peers and are not controlled by this field.
        pub export_subnet_routes_with_public_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to import the custom routes from the peer network. Defaults to `false`.
        pub import_custom_routes: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether subnet routes with public IP range are imported. The default value is false. The IPv4 special-use ranges (https://en.wikipedia.org/wiki/IPv4#Special_addresses) are always imported from peers and are not controlled by this field.
        pub import_subnet_routes_with_public_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the peering.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary network of the peering.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The peer network in the peering. The peer network
        /// may belong to a different project.
        pub peer_network: pulumi_wasm_rust::Output<String>,
        /// Which IP version(s) of traffic and routes are allowed to be imported or exported between peer networks. The default value is IPV4_ONLY. Possible values: ["IPV4_ONLY", "IPV4_IPV6"].
        pub stack_type: pulumi_wasm_rust::Output<Option<String>>,
        /// State for the peering, either `ACTIVE` or `INACTIVE`. The peering is
        /// `ACTIVE` when there's a matching configuration in the peer network.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Details about the current state of the peering.
        pub state_details: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NetworkPeeringArgs,
    ) -> NetworkPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let export_custom_routes_binding = args
            .export_custom_routes
            .get_output(context)
            .get_inner();
        let export_subnet_routes_with_public_ip_binding = args
            .export_subnet_routes_with_public_ip
            .get_output(context)
            .get_inner();
        let import_custom_routes_binding = args
            .import_custom_routes
            .get_output(context)
            .get_inner();
        let import_subnet_routes_with_public_ip_binding = args
            .import_subnet_routes_with_public_ip
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let peer_network_binding = args.peer_network.get_output(context).get_inner();
        let stack_type_binding = args.stack_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/networkPeering:NetworkPeering".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "exportCustomRoutes".into(),
                    value: &export_custom_routes_binding,
                },
                register_interface::ObjectField {
                    name: "exportSubnetRoutesWithPublicIp".into(),
                    value: &export_subnet_routes_with_public_ip_binding,
                },
                register_interface::ObjectField {
                    name: "importCustomRoutes".into(),
                    value: &import_custom_routes_binding,
                },
                register_interface::ObjectField {
                    name: "importSubnetRoutesWithPublicIp".into(),
                    value: &import_subnet_routes_with_public_ip_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "peerNetwork".into(),
                    value: &peer_network_binding,
                },
                register_interface::ObjectField {
                    name: "stackType".into(),
                    value: &stack_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkPeeringResult {
            export_custom_routes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exportCustomRoutes"),
            ),
            export_subnet_routes_with_public_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exportSubnetRoutesWithPublicIp"),
            ),
            import_custom_routes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("importCustomRoutes"),
            ),
            import_subnet_routes_with_public_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("importSubnetRoutesWithPublicIp"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            peer_network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerNetwork"),
            ),
            stack_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stackType"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            state_details: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stateDetails"),
            ),
        }
    }
}
