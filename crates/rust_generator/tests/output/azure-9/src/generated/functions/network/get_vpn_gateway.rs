#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpn_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpnGatewayArgs {
        /// The Name of the VPN Gateway.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the VPN Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVpnGatewayResult {
        /// A `bgp_settings` block as defined below.
        pub bgp_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetVpnGatewayBgpSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the VPN Gateway exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Scale Unit of this VPN Gateway.
        pub scale_unit: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags assigned to the VPN Gateway.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the Virtual Hub within which this VPN Gateway has been created.
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpnGatewayArgs,
    ) -> GetVpnGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getVpnGateway:getVpnGateway".into(),
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
        GetVpnGatewayResult {
            bgp_settings: o.get_field("bgpSettings"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scale_unit: o.get_field("scaleUnit"),
            tags: o.get_field("tags"),
            virtual_hub_id: o.get_field("virtualHubId"),
        }
    }
}
