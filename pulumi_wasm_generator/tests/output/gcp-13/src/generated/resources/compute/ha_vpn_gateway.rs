/// Represents a VPN gateway running in GCP. This virtual device is managed
/// by Google, but used only by you. This type of VPN Gateway allows for the creation
/// of VPN solutions with higher availability than classic Target VPN Gateways.
///
///
/// To get more information about HaVpnGateway, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/vpnGateways)
/// * How-to Guides
///     * [Choosing a VPN](https://cloud.google.com/vpn/docs/how-to/choosing-a-vpn)
///     * [Cloud VPN Overview](https://cloud.google.com/vpn/docs/concepts/overview)
///
/// ## Example Usage
///
/// ### Ha Vpn Gateway Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let haGateway1 = ha_vpn_gateway::create(
///         "haGateway1",
///         HaVpnGatewayArgs::builder()
///             .name("ha-vpn-1")
///             .network("${network1.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let network1 = network::create(
///         "network1",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("network1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Ha Vpn Gateway Ipv6
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let haGateway1 = ha_vpn_gateway::create(
///         "haGateway1",
///         HaVpnGatewayArgs::builder()
///             .name("ha-vpn-1")
///             .network("${network1.id}")
///             .region("us-central1")
///             .stack_type("IPV4_IPV6")
///             .build_struct(),
///     );
///     let network1 = network::create(
///         "network1",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("network1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Compute Ha Vpn Gateway Encrypted Interconnect
///
///
/// ```yaml
/// resources:
///   vpn-gateway:
///     type: gcp:compute:HaVpnGateway
///     properties:
///       name: test-ha-vpngw
///       network: ${network.id}
///       vpnInterfaces:
///         - id: 0
///           interconnectAttachment: ${attachment1.selfLink}
///         - id: 1
///           interconnectAttachment: ${attachment2.selfLink}
///   attachment1:
///     type: gcp:compute:InterconnectAttachment
///     properties:
///       name: test-interconnect-attachment1
///       edgeAvailabilityDomain: AVAILABILITY_DOMAIN_1
///       type: PARTNER
///       router: ${router.id}
///       encryption: IPSEC
///       ipsecInternalAddresses:
///         - ${address1.selfLink}
///   attachment2:
///     type: gcp:compute:InterconnectAttachment
///     properties:
///       name: test-interconnect-attachment2
///       edgeAvailabilityDomain: AVAILABILITY_DOMAIN_2
///       type: PARTNER
///       router: ${router.id}
///       encryption: IPSEC
///       ipsecInternalAddresses:
///         - ${address2.selfLink}
///   address1:
///     type: gcp:compute:Address
///     properties:
///       name: test-address1
///       addressType: INTERNAL
///       purpose: IPSEC_INTERCONNECT
///       address: 192.168.1.0
///       prefixLength: 29
///       network: ${network.selfLink}
///   address2:
///     type: gcp:compute:Address
///     properties:
///       name: test-address2
///       addressType: INTERNAL
///       purpose: IPSEC_INTERCONNECT
///       address: 192.168.2.0
///       prefixLength: 29
///       network: ${network.selfLink}
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: test-router
///       network: ${network.name}
///       encryptedInterconnectRouter: true
///       bgp:
///         asn: 16550
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: test-network
///       autoCreateSubnetworks: false
/// ```
///
/// ## Import
///
/// HaVpnGateway can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/vpnGateways/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, HaVpnGateway can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/haVpnGateway:HaVpnGateway default projects/{{project}}/regions/{{region}}/vpnGateways/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/haVpnGateway:HaVpnGateway default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/haVpnGateway:HaVpnGateway default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/haVpnGateway:HaVpnGateway default {{name}}
/// ```
///
pub mod ha_vpn_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HaVpnGatewayArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP family of the gateway IPs for the HA-VPN gateway interfaces. If not specified, IPV4 will be used.
        /// Default value is `IPV4`.
        /// Possible values are: `IPV4`, `IPV6`.
        #[builder(into, default)]
        pub gateway_ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The network this VPN gateway is accepting traffic for.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region this gateway should sit in.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The stack type for this VPN gateway to identify the IP protocols that are enabled.
        /// If not specified, IPV4_ONLY will be used.
        /// Default value is `IPV4_ONLY`.
        /// Possible values are: `IPV4_ONLY`, `IPV4_IPV6`, `IPV6_ONLY`.
        #[builder(into, default)]
        pub stack_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of interfaces on this VPN gateway.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vpn_interfaces: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::HaVpnGatewayVpnInterface>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HaVpnGatewayResult {
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP family of the gateway IPs for the HA-VPN gateway interfaces. If not specified, IPV4 will be used.
        /// Default value is `IPV4`.
        /// Possible values are: `IPV4`, `IPV6`.
        pub gateway_ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The network this VPN gateway is accepting traffic for.
        ///
        ///
        /// - - -
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region this gateway should sit in.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The stack type for this VPN gateway to identify the IP protocols that are enabled.
        /// If not specified, IPV4_ONLY will be used.
        /// Default value is `IPV4_ONLY`.
        /// Possible values are: `IPV4_ONLY`, `IPV4_IPV6`, `IPV6_ONLY`.
        pub stack_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of interfaces on this VPN gateway.
        /// Structure is documented below.
        pub vpn_interfaces: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::HaVpnGatewayVpnInterface>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HaVpnGatewayArgs) -> HaVpnGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let gateway_ip_version_binding = args.gateway_ip_version.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let stack_type_binding = args.stack_type.get_inner();
        let vpn_interfaces_binding = args.vpn_interfaces.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/haVpnGateway:HaVpnGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayIpVersion".into(),
                    value: &gateway_ip_version_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "stackType".into(),
                    value: &stack_type_binding,
                },
                register_interface::ObjectField {
                    name: "vpnInterfaces".into(),
                    value: &vpn_interfaces_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "gatewayIpVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "stackType".into(),
                },
                register_interface::ResultField {
                    name: "vpnInterfaces".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HaVpnGatewayResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            gateway_ip_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayIpVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            stack_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackType").unwrap(),
            ),
            vpn_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnInterfaces").unwrap(),
            ),
        }
    }
}
