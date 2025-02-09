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
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualNetworkGatewayArgs,
    ) -> GetVirtualNetworkGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getVirtualNetworkGateway:getVirtualNetworkGateway"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVirtualNetworkGatewayResult {
            active_active: o.get_field("activeActive"),
            bgp_settings: o.get_field("bgpSettings"),
            custom_routes: o.get_field("customRoutes"),
            default_local_network_gateway_id: o
                .get_field("defaultLocalNetworkGatewayId"),
            enable_bgp: o.get_field("enableBgp"),
            generation: o.get_field("generation"),
            id: o.get_field("id"),
            ip_configurations: o.get_field("ipConfigurations"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_ip_address_enabled: o.get_field("privateIpAddressEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            vpn_client_configurations: o.get_field("vpnClientConfigurations"),
            vpn_type: o.get_field("vpnType"),
        }
    }
}
