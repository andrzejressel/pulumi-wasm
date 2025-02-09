#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualGatewayArgs {
        /// Name of the service mesh in which the virtual gateway exists.
        #[builder(into)]
        pub mesh_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the virtual gateway.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVirtualGatewayResult {
        /// ARN of the virtual gateway.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the virtual gateway.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Last update date of the virtual gateway.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        pub mesh_name: pulumi_gestalt_rust::Output<String>,
        pub mesh_owner: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// Virtual gateway specification. See the `aws.appmesh.VirtualGateway` resource for details.
        pub specs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appmesh::GetVirtualGatewaySpec>,
        >,
        /// Map of tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVirtualGatewayArgs,
    ) -> GetVirtualGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mesh_name_binding_1 = args.mesh_name.get_output(context);
        let mesh_name_binding = mesh_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:appmesh/getVirtualGateway:getVirtualGateway".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "meshName".into(),
                    value: &mesh_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVirtualGatewayResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            last_updated_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedDate"),
            ),
            mesh_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("meshName"),
            ),
            mesh_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("meshOwner"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceOwner"),
            ),
            specs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("specs")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
