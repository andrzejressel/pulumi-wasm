pub mod get_vpn_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpnGatewayArgs {
        /// The Name of the VPN Gateway.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the VPN Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVpnGatewayResult {
        /// A `bgp_settings` block as defined below.
        pub bgp_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetVpnGatewayBgpSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the VPN Gateway exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Scale Unit of this VPN Gateway.
        pub scale_unit: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags assigned to the VPN Gateway.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the Virtual Hub within which this VPN Gateway has been created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVpnGatewayArgs,
    ) -> GetVpnGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVpnGateway:getVpnGateway".into(),
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
        GetVpnGatewayResult {
            bgp_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bgpSettings"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scale_unit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scaleUnit"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualHubId"),
            ),
        }
    }
}
