#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_network_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualNetworkGatewayArgs {
        /// Specifies the name of the Virtual Network Gateway.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Virtual Network Gateway is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualNetworkGatewayResult {
        /// Is this an Active-Active Gateway?
        pub active_active: pulumi_gestalt_rust::Output<bool>,
        pub bgp_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetVirtualNetworkGatewayBgpSetting>,
        >,
        pub custom_routes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetVirtualNetworkGatewayCustomRoute>,
        >,
        /// The ID of the local network gateway
        /// through which outbound Internet traffic from the virtual network in which the
        /// gateway is created will be routed (*forced tunneling*). Refer to the
        /// [Azure documentation on forced tunneling](https://docs.microsoft.com/azure/vpn-gateway/vpn-gateway-forced-tunneling-rm).
        pub default_local_network_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// Will BGP (Border Gateway Protocol) will be enabled
        /// for this Virtual Network Gateway.
        pub enable_bgp: pulumi_gestalt_rust::Output<bool>,
        /// The Generation of the Virtual Network Gateway.
        pub generation: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// One or two `ip_configuration` blocks documented below.
        pub ip_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetVirtualNetworkGatewayIpConfiguration,
            >,
        >,
        /// The location/region where the Virtual Network Gateway is located.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The user-defined name of the root certificate.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether a private IP will be used for this  gateway for connections.
        pub private_ip_address_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration of the size and capacity of the Virtual Network Gateway.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The type of the Virtual Network Gateway.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// A `vpn_client_configuration` block which is documented below.
        pub vpn_client_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetVirtualNetworkGatewayVpnClientConfiguration,
            >,
        >,
        /// The routing type of the Virtual Network Gateway.
        pub vpn_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVirtualNetworkGatewayArgs,
    ) -> GetVirtualNetworkGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualNetworkGateway:getVirtualNetworkGateway"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVirtualNetworkGatewayResult {
            active_active: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activeActive"),
            ),
            bgp_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpSettings"),
            ),
            custom_routes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customRoutes"),
            ),
            default_local_network_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultLocalNetworkGatewayId"),
            ),
            enable_bgp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableBgp"),
            ),
            generation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("generation"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipConfigurations"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_ip_address_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddressEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            vpn_client_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpnClientConfigurations"),
            ),
            vpn_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpnType"),
            ),
        }
    }
}
