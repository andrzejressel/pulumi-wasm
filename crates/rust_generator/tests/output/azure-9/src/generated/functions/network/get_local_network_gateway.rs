#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetLocalNetworkGatewayArgs,
    ) -> GetLocalNetworkGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getLocalNetworkGateway:getLocalNetworkGateway".into(),
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
        GetLocalNetworkGatewayResult {
            address_spaces: o.get_field("addressSpaces"),
            bgp_settings: o.get_field("bgpSettings"),
            gateway_address: o.get_field("gatewayAddress"),
            gateway_fqdn: o.get_field("gatewayFqdn"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
