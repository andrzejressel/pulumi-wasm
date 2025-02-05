pub mod get_mesh {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMeshArgs {
        /// AWS account ID of the service mesh's owner.
        #[builder(into, default)]
        pub mesh_owner: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the service mesh.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetMeshResult {
        /// ARN of the service mesh.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Creation date of the service mesh.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Last update date of the service mesh.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        pub mesh_owner: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        /// Service mesh specification. See the `aws.appmesh.Mesh` resource for details.
        pub specs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appmesh::GetMeshSpec>,
        >,
        /// Map of tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetMeshArgs,
    ) -> GetMeshResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mesh_owner_binding = args.mesh_owner.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:appmesh/getMesh:getMesh".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "meshOwner".into(),
                    value: &mesh_owner_binding,
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
        GetMeshResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedDate"),
            ),
            mesh_owner: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("meshOwner"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_owner: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceOwner"),
            ),
            specs: pulumi_wasm_rust::__private::into_domain(o.extract_field("specs")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
