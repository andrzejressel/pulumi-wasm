#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service_endpoint_connections {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceEndpointConnectionsArgs {
        /// The name of the resource group in which the private link service resides.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the private link service.
        #[builder(into)]
        pub service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceEndpointConnectionsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub private_endpoint_connections: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privatelink::GetServiceEndpointConnectionsPrivateEndpointConnection,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub service_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the private link service.
        pub service_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceEndpointConnectionsArgs,
    ) -> GetServiceEndpointConnectionsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_id_binding = args.service_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:privatelink/getServiceEndpointConnections:getServiceEndpointConnections"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceId".into(),
                    value: service_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceEndpointConnectionsResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            private_endpoint_connections: o.get_field("privateEndpointConnections"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_id: o.get_field("serviceId"),
            service_name: o.get_field("serviceName"),
        }
    }
}
