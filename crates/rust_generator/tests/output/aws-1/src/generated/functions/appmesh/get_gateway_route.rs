#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_gateway_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayRouteArgs {
        /// Name of the service mesh in which the virtual gateway exists.
        #[builder(into)]
        pub mesh_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID of the service mesh's owner.
        #[builder(into, default)]
        pub mesh_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the gateway route.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the virtual gateway in which the route exists.
        #[builder(into)]
        pub virtual_gateway_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayRouteResult {
        /// ARN of the gateway route.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the gateway route.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Last update date of the gateway route.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        pub mesh_name: pulumi_gestalt_rust::Output<String>,
        pub mesh_owner: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// Gateway route specification. See the `aws.appmesh.GatewayRoute` resource for details.
        pub specs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appmesh::GetGatewayRouteSpec>,
        >,
        /// Map of tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub virtual_gateway_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGatewayRouteArgs,
    ) -> GetGatewayRouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mesh_name_binding = args.mesh_name.get_output(context);
        let mesh_owner_binding = args.mesh_owner.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_gateway_name_binding = args.virtual_gateway_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:appmesh/getGatewayRoute:getGatewayRoute".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshName".into(),
                    value: &mesh_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshOwner".into(),
                    value: &mesh_owner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualGatewayName".into(),
                    value: &virtual_gateway_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGatewayRouteResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            id: o.get_field("id"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            mesh_name: o.get_field("meshName"),
            mesh_owner: o.get_field("meshOwner"),
            name: o.get_field("name"),
            resource_owner: o.get_field("resourceOwner"),
            specs: o.get_field("specs"),
            tags: o.get_field("tags"),
            virtual_gateway_name: o.get_field("virtualGatewayName"),
        }
    }
}
