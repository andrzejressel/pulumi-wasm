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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ha_vpn_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HaVpnGatewayArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP family of the gateway IPs for the HA-VPN gateway interfaces. If not specified, IPV4 will be used.
        /// Default value is `IPV4`.
        /// Possible values are: `IPV4`, `IPV6`.
        #[builder(into, default)]
        pub gateway_ip_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network this VPN gateway is accepting traffic for.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region this gateway should sit in.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The stack type for this VPN gateway to identify the IP protocols that are enabled.
        /// If not specified, IPV4_ONLY will be used.
        /// Default value is `IPV4_ONLY`.
        /// Possible values are: `IPV4_ONLY`, `IPV4_IPV6`, `IPV6_ONLY`.
        #[builder(into, default)]
        pub stack_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of interfaces on this VPN gateway.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vpn_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::HaVpnGatewayVpnInterface>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HaVpnGatewayResult {
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IP family of the gateway IPs for the HA-VPN gateway interfaces. If not specified, IPV4 will be used.
        /// Default value is `IPV4`.
        /// Possible values are: `IPV4`, `IPV6`.
        pub gateway_ip_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The network this VPN gateway is accepting traffic for.
        ///
        ///
        /// - - -
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region this gateway should sit in.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The stack type for this VPN gateway to identify the IP protocols that are enabled.
        /// If not specified, IPV4_ONLY will be used.
        /// Default value is `IPV4_ONLY`.
        /// Possible values are: `IPV4_ONLY`, `IPV4_IPV6`, `IPV6_ONLY`.
        pub stack_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of interfaces on this VPN gateway.
        /// Structure is documented below.
        pub vpn_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::HaVpnGatewayVpnInterface>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HaVpnGatewayArgs,
    ) -> HaVpnGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let gateway_ip_version_binding = args.gateway_ip_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let stack_type_binding = args.stack_type.get_output(context);
        let vpn_interfaces_binding = args.vpn_interfaces.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/haVpnGateway:HaVpnGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayIpVersion".into(),
                    value: &gateway_ip_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackType".into(),
                    value: &stack_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnInterfaces".into(),
                    value: &vpn_interfaces_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HaVpnGatewayResult {
            description: o.get_field("description"),
            gateway_ip_version: o.get_field("gatewayIpVersion"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            stack_type: o.get_field("stackType"),
            vpn_interfaces: o.get_field("vpnInterfaces"),
        }
    }
}
