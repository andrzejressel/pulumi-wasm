/// Represents a VPN gateway running in GCP. This virtual device is managed
/// by Google, but used only by you.
///
///
/// To get more information about VpnGateway, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/targetVpnGateways)
///
/// > **Warning:** Classic VPN is deprecating certain functionality on October 31, 2021. For more information,
/// see the [Classic VPN partial deprecation page](https://cloud.google.com/network-connectivity/docs/vpn/deprecations/classic-vpn-deprecation).
///
/// ## Example Usage
///
/// ### Target Vpn Gateway Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let frEsp = forwarding_rule::create(
///         "frEsp",
///         ForwardingRuleArgs::builder()
///             .ip_address("${vpnStaticIp.address}")
///             .ip_protocol("ESP")
///             .name("fr-esp")
///             .target("${targetGateway.id}")
///             .build_struct(),
///     );
///     let frUdp4500 = forwarding_rule::create(
///         "frUdp4500",
///         ForwardingRuleArgs::builder()
///             .ip_address("${vpnStaticIp.address}")
///             .ip_protocol("UDP")
///             .name("fr-udp4500")
///             .port_range("4500")
///             .target("${targetGateway.id}")
///             .build_struct(),
///     );
///     let frUdp500 = forwarding_rule::create(
///         "frUdp500",
///         ForwardingRuleArgs::builder()
///             .ip_address("${vpnStaticIp.address}")
///             .ip_protocol("UDP")
///             .name("fr-udp500")
///             .port_range("500")
///             .target("${targetGateway.id}")
///             .build_struct(),
///     );
///     let network1 = network::create(
///         "network1",
///         NetworkArgs::builder().name("network-1").build_struct(),
///     );
///     let route1 = route::create(
///         "route1",
///         RouteArgs::builder()
///             .dest_range("15.0.0.0/24")
///             .name("route1")
///             .network("${network1.name}")
///             .next_hop_vpn_tunnel("${tunnel1.id}")
///             .priority(1000)
///             .build_struct(),
///     );
///     let targetGateway = vpn_gateway::create(
///         "targetGateway",
///         VpnGatewayArgs::builder().name("vpn-1").network("${network1.id}").build_struct(),
///     );
///     let tunnel1 = vpn_tunnel::create(
///         "tunnel1",
///         VpnTunnelArgs::builder()
///             .name("tunnel1")
///             .peer_ip("15.0.0.120")
///             .shared_secret("a secret message")
///             .target_vpn_gateway("${targetGateway.id}")
///             .build_struct(),
///     );
///     let vpnStaticIp = address::create(
///         "vpnStaticIp",
///         AddressArgs::builder().name("vpn-static-ip").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VpnGateway can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/targetVpnGateways/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, VpnGateway can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/vPNGateway:VPNGateway default projects/{{project}}/regions/{{region}}/targetVpnGateways/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/vPNGateway:VPNGateway default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/vPNGateway:VPNGateway default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/vPNGateway:VPNGateway default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod vpn_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VPNGatewayArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
    }
    #[allow(dead_code)]
    pub struct VPNGatewayResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the resource.
        pub gateway_id: pulumi_gestalt_rust::Output<i32>,
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
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VPNGatewayArgs,
    ) -> VPNGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/vPNGateway:VPNGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VPNGatewayResult {
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
