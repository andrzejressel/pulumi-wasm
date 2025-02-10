#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_endpoint_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEndpointConnectionArgs {
        /// Specifies the Name of the private endpoint.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Name of the Resource Group within which the private endpoint exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEndpointConnectionResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The supported Azure location where the resource exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the private endpoint.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privatelink::GetEndpointConnectionNetworkInterface,
            >,
        >,
        pub private_service_connections: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privatelink::GetEndpointConnectionPrivateServiceConnection,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEndpointConnectionArgs,
    ) -> GetEndpointConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:privatelink/getEndpointConnection:getEndpointConnection"
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
        GetEndpointConnectionResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            private_service_connections: o.get_field("privateServiceConnections"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
