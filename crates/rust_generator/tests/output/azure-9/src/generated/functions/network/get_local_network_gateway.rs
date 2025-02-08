#[allow(clippy::doc_lazy_continuation)]
pub mod get_local_network_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalNetworkGatewayArgs {
        /// The name of the Local Network Gateway.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Local Network Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocalNetworkGatewayResult {
        /// The list of string CIDRs representing the address spaces the gateway exposes.
        pub address_spaces: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `bgp_settings` block as defined below containing the Local Network Gateway's BGP speaker settings.
        pub bgp_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetLocalNetworkGatewayBgpSetting>,
        >,
        /// The gateway IP address the Local Network Gateway uses.
        pub gateway_address: pulumi_gestalt_rust::Output<String>,
        /// The gateway FQDN the Local Network Gateway uses.
        pub gateway_fqdn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Local Network Gateway exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Local Network Gateway.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLocalNetworkGatewayArgs,
    ) -> GetLocalNetworkGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getLocalNetworkGateway:getLocalNetworkGateway".into(),
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
        GetLocalNetworkGatewayResult {
            address_spaces: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addressSpaces"),
            ),
            bgp_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpSettings"),
            ),
            gateway_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayAddress"),
            ),
            gateway_fqdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayFqdn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
