/// Manages a VPN Gateway Connection.
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
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.0.0/24")
///             .location("${example.location}")
///             .name("example-hub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-vwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVpnGateway = vpn_gateway::create(
///         "exampleVpnGateway",
///         VpnGatewayArgs::builder()
///             .location("${example.location}")
///             .name("example-vpng")
///             .resource_group_name("${example.name}")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVpnGatewayConnection = vpn_gateway_connection::create(
///         "exampleVpnGatewayConnection",
///         VpnGatewayConnectionArgs::builder()
///             .name("example")
///             .remote_vpn_site_id("${exampleVpnSite.id}")
///             .vpn_gateway_id("${exampleVpnGateway.id}")
///             .vpn_links(
///                 vec![
///                     VpnGatewayConnectionVpnLink::builder().name("link1")
///                     .vpnSiteLinkId("${exampleVpnSite.links[0].id}").build_struct(),
///                     VpnGatewayConnectionVpnLink::builder().name("link2")
///                     .vpnSiteLinkId("${exampleVpnSite.links[1].id}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleVpnSite = vpn_site::create(
///         "exampleVpnSite",
///         VpnSiteArgs::builder()
///             .links(
///                 vec![
///                     VpnSiteLink::builder().ipAddress("10.1.0.0").name("link1")
///                     .build_struct(), VpnSiteLink::builder().ipAddress("10.2.0.0")
///                     .name("link2").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-vpn-site")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VPN Gateway Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/vpnGatewayConnection:VpnGatewayConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/vpnGateways/gateway1/vpnConnections/conn1
/// ```
///
pub mod vpn_gateway_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnGatewayConnectionArgs {
        /// Whether Internet Security is enabled for this VPN Connection. Defaults to `false`.
        #[builder(into, default)]
        pub internet_security_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this VPN Gateway Connection. Changing this forces a new VPN Gateway Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the remote VPN Site, which will connect to the VPN Gateway. Changing this forces a new VPN Gateway Connection to be created.
        #[builder(into)]
        pub remote_vpn_site_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `routing` block as defined below. If this is not specified, there will be a default route table created implicitly.
        #[builder(into, default)]
        pub routing: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::VpnGatewayConnectionRouting>,
        >,
        /// One or more `traffic_selector_policy` blocks as defined below.
        #[builder(into, default)]
        pub traffic_selector_policies: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::VpnGatewayConnectionTrafficSelectorPolicy,
                >,
            >,
        >,
        /// The ID of the VPN Gateway that this VPN Gateway Connection belongs to. Changing this forces a new VPN Gateway Connection to be created.
        #[builder(into)]
        pub vpn_gateway_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// One or more `vpn_link` blocks as defined below.
        #[builder(into)]
        pub vpn_links: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::network::VpnGatewayConnectionVpnLink>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpnGatewayConnectionResult {
        /// Whether Internet Security is enabled for this VPN Connection. Defaults to `false`.
        pub internet_security_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this VPN Gateway Connection. Changing this forces a new VPN Gateway Connection to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the remote VPN Site, which will connect to the VPN Gateway. Changing this forces a new VPN Gateway Connection to be created.
        pub remote_vpn_site_id: pulumi_wasm_rust::Output<String>,
        /// A `routing` block as defined below. If this is not specified, there will be a default route table created implicitly.
        pub routing: pulumi_wasm_rust::Output<
            super::super::types::network::VpnGatewayConnectionRouting,
        >,
        /// One or more `traffic_selector_policy` blocks as defined below.
        pub traffic_selector_policies: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::VpnGatewayConnectionTrafficSelectorPolicy,
                >,
            >,
        >,
        /// The ID of the VPN Gateway that this VPN Gateway Connection belongs to. Changing this forces a new VPN Gateway Connection to be created.
        pub vpn_gateway_id: pulumi_wasm_rust::Output<String>,
        /// One or more `vpn_link` blocks as defined below.
        pub vpn_links: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::VpnGatewayConnectionVpnLink>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpnGatewayConnectionArgs,
    ) -> VpnGatewayConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let internet_security_enabled_binding = args
            .internet_security_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let remote_vpn_site_id_binding = args
            .remote_vpn_site_id
            .get_output(context)
            .get_inner();
        let routing_binding = args.routing.get_output(context).get_inner();
        let traffic_selector_policies_binding = args
            .traffic_selector_policies
            .get_output(context)
            .get_inner();
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_output(context).get_inner();
        let vpn_links_binding = args.vpn_links.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/vpnGatewayConnection:VpnGatewayConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "internetSecurityEnabled".into(),
                    value: &internet_security_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "remoteVpnSiteId".into(),
                    value: &remote_vpn_site_id_binding,
                },
                register_interface::ObjectField {
                    name: "routing".into(),
                    value: &routing_binding,
                },
                register_interface::ObjectField {
                    name: "trafficSelectorPolicies".into(),
                    value: &traffic_selector_policies_binding,
                },
                register_interface::ObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpnLinks".into(),
                    value: &vpn_links_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpnGatewayConnectionResult {
            internet_security_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("internetSecurityEnabled"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            remote_vpn_site_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("remoteVpnSiteId"),
            ),
            routing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("routing"),
            ),
            traffic_selector_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trafficSelectorPolicies"),
            ),
            vpn_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnGatewayId"),
            ),
            vpn_links: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnLinks"),
            ),
        }
    }
}
