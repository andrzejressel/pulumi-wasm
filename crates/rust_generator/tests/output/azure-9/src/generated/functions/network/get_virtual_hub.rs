#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualHubArgs {
        /// The name of the Virtual Hub.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Virtual Hub exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualHubResult {
        /// The Address Prefix used for this Virtual Hub.
        pub address_prefix: pulumi_gestalt_rust::Output<String>,
        /// The ID of the default Route Table in the Virtual Hub.
        pub default_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Virtual Hub exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Virtual Hub.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Autonomous System Number of the Virtual Hub BGP router.
        pub virtual_router_asn: pulumi_gestalt_rust::Output<i32>,
        /// The IP addresses of the Virtual Hub BGP router.
        pub virtual_router_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the Virtual WAN within which the Virtual Hub exists.
        pub virtual_wan_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualHubArgs,
    ) -> GetVirtualHubResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getVirtualHub:getVirtualHub".into(),
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
        GetVirtualHubResult {
            address_prefix: o.get_field("addressPrefix"),
            default_route_table_id: o.get_field("defaultRouteTableId"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            virtual_router_asn: o.get_field("virtualRouterAsn"),
            virtual_router_ips: o.get_field("virtualRouterIps"),
            virtual_wan_id: o.get_field("virtualWanId"),
        }
    }
}
