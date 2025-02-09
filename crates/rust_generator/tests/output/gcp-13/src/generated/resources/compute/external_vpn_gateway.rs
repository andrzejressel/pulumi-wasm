/// Represents a VPN gateway managed outside of GCP.
///
///
/// To get more information about ExternalVpnGateway, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/externalVpnGateways)
///
/// ## Example Usage
///
/// ### External Vpn Gateway
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let externalGateway = external_vpn_gateway::create(
///         "externalGateway",
///         ExternalVpnGatewayArgs::builder()
///             .description("An externally managed VPN gateway")
///             .interfaces(
///                 vec![
///                     ExternalVpnGatewayInterface::builder().id(0).ipAddress("8.8.8.8")
///                     .build_struct(),
///                 ],
///             )
///             .name("external-gateway")
///             .redundancy_type("SINGLE_IP_INTERNALLY_REDUNDANT")
///             .build_struct(),
///     );
///     let haGateway = ha_vpn_gateway::create(
///         "haGateway",
///         HaVpnGatewayArgs::builder()
///             .name("ha-vpn")
///             .network("${network.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let network = network::create(
///         "network",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("network-1")
///             .routing_mode("GLOBAL")
///             .build_struct(),
///     );
///     let networkSubnet1 = subnetwork::create(
///         "networkSubnet1",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.1.0/24")
///             .name("ha-vpn-subnet-1")
///             .network("${network.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let networkSubnet2 = subnetwork::create(
///         "networkSubnet2",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.2.0/24")
///             .name("ha-vpn-subnet-2")
///             .network("${network.id}")
///             .region("us-west1")
///             .build_struct(),
///     );
///     let router1 = router::create(
///         "router1",
///         RouterArgs::builder()
///             .bgp(RouterBgp::builder().asn(64514).build_struct())
///             .name("ha-vpn-router1")
///             .network("${network.name}")
///             .build_struct(),
///     );
///     let router1Interface1 = router_interface::create(
///         "router1Interface1",
///         RouterInterfaceArgs::builder()
///             .ip_range("169.254.0.1/30")
///             .name("router1-interface1")
///             .region("us-central1")
///             .router("${router1.name}")
///             .vpn_tunnel("${tunnel1.name}")
///             .build_struct(),
///     );
///     let router1Interface2 = router_interface::create(
///         "router1Interface2",
///         RouterInterfaceArgs::builder()
///             .ip_range("169.254.1.1/30")
///             .name("router1-interface2")
///             .region("us-central1")
///             .router("${router1.name}")
///             .vpn_tunnel("${tunnel2.name}")
///             .build_struct(),
///     );
///     let router1Peer1 = router_peer::create(
///         "router1Peer1",
///         RouterPeerArgs::builder()
///             .advertised_route_priority(100)
///             .interface("${router1Interface1.name}")
///             .name("router1-peer1")
///             .peer_asn(64515)
///             .peer_ip_address("169.254.0.2")
///             .region("us-central1")
///             .router("${router1.name}")
///             .build_struct(),
///     );
///     let router1Peer2 = router_peer::create(
///         "router1Peer2",
///         RouterPeerArgs::builder()
///             .advertised_route_priority(100)
///             .interface("${router1Interface2.name}")
///             .name("router1-peer2")
///             .peer_asn(64515)
///             .peer_ip_address("169.254.1.2")
///             .region("us-central1")
///             .router("${router1.name}")
///             .build_struct(),
///     );
///     let tunnel1 = vpn_tunnel::create(
///         "tunnel1",
///         VpnTunnelArgs::builder()
///             .name("ha-vpn-tunnel1")
///             .peer_external_gateway("${externalGateway.id}")
///             .peer_external_gateway_interface(0)
///             .region("us-central1")
///             .router("${router1.id}")
///             .shared_secret("a secret message")
///             .vpn_gateway("${haGateway.id}")
///             .vpn_gateway_interface(0)
///             .build_struct(),
///     );
///     let tunnel2 = vpn_tunnel::create(
///         "tunnel2",
///         VpnTunnelArgs::builder()
///             .name("ha-vpn-tunnel2")
///             .peer_external_gateway("${externalGateway.id}")
///             .peer_external_gateway_interface(0)
///             .region("us-central1")
///             .router(" ${router1.id}")
///             .shared_secret("a secret message")
///             .vpn_gateway("${haGateway.id}")
///             .vpn_gateway_interface(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ExternalVpnGateway can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/externalVpnGateways/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ExternalVpnGateway can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/externalVpnGateway:ExternalVpnGateway default projects/{{project}}/global/externalVpnGateways/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/externalVpnGateway:ExternalVpnGateway default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/externalVpnGateway:ExternalVpnGateway default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod external_vpn_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExternalVpnGatewayArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of interfaces on this external VPN gateway.
        /// Structure is documented below.
        #[builder(into, default)]
        pub interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::ExternalVpnGatewayInterface>>,
        >,
        /// Labels for the external VPN gateway resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates the redundancy type of this external VPN gateway
        /// Possible values are: `FOUR_IPS_REDUNDANCY`, `SINGLE_IP_INTERNALLY_REDUNDANT`, `TWO_IPS_REDUNDANCY`.
        #[builder(into, default)]
        pub redundancy_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ExternalVpnGatewayResult {
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of interfaces on this external VPN gateway.
        /// Structure is documented below.
        pub interfaces: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::ExternalVpnGatewayInterface>>,
        >,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Labels for the external VPN gateway resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates the redundancy type of this external VPN gateway
        /// Possible values are: `FOUR_IPS_REDUNDANCY`, `SINGLE_IP_INTERNALLY_REDUNDANT`, `TWO_IPS_REDUNDANCY`.
        pub redundancy_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ExternalVpnGatewayArgs,
    ) -> ExternalVpnGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let interfaces_binding_1 = args.interfaces.get_output(context);
        let interfaces_binding = interfaces_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let redundancy_type_binding_1 = args.redundancy_type.get_output(context);
        let redundancy_type_binding = redundancy_type_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/externalVpnGateway:ExternalVpnGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "interfaces".into(),
                    value: &interfaces_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "redundancyType".into(),
                    value: &redundancy_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExternalVpnGatewayResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            interfaces: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interfaces"),
            ),
            label_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            redundancy_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redundancyType"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
