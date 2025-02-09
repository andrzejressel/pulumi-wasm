#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_hub_route_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualHubRouteTableArgs {
        /// The name of the Virtual Hub Route Table.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Virtual Hub Route Table exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for Virtual Hub Route Table.
        #[builder(into)]
        pub virtual_hub_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualHubRouteTableResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of labels associated with this route table.
        pub labels: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name which is used for this route.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `route` block as defined below.
        pub routes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetVirtualHubRouteTableRoute>,
        >,
        /// The ID of the Virtual Hub within which this route table is created
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
        pub virtual_hub_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualHubRouteTableArgs,
    ) -> GetVirtualHubRouteTableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let virtual_hub_name_binding = args.virtual_hub_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getVirtualHubRouteTable:getVirtualHubRouteTable"
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubName".into(),
                    value: virtual_hub_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVirtualHubRouteTableResult {
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            routes: o.get_field("routes"),
            virtual_hub_id: o.get_field("virtualHubId"),
            virtual_hub_name: o.get_field("virtualHubName"),
        }
    }
}
