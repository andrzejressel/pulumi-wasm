/// Provides an AWS App Mesh service mesh resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let simple = mesh::create(
///         "simple",
///         MeshArgs::builder().name("simpleapp").build_struct(),
///     );
/// }
/// ```
///
/// ### Egress Filter
///
/// ```yaml
/// resources:
///   simple:
///     type: aws:appmesh:Mesh
///     properties:
///       name: simpleapp
///       spec:
///         egressFilter:
///           type: ALLOW_ALL
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Mesh service meshes using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appmesh/mesh:Mesh simple simpleapp
/// ```
pub mod mesh {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MeshArgs {
        /// Name to use for the service mesh. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Service mesh specification to apply.
        #[builder(into, default)]
        pub spec: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appmesh::MeshSpec>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MeshResult {
        /// ARN of the service mesh.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Creation date of the service mesh.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Last update date of the service mesh.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the service mesh's owner.
        pub mesh_owner: pulumi_wasm_rust::Output<String>,
        /// Name to use for the service mesh. Must be between 1 and 255 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        /// Service mesh specification to apply.
        pub spec: pulumi_wasm_rust::Output<
            Option<super::super::types::appmesh::MeshSpec>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MeshArgs,
    ) -> MeshResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let spec_binding = args.spec.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appmesh/mesh:Mesh".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "spec".into(),
                    value: &spec_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MeshResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
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
            spec: pulumi_wasm_rust::__private::into_domain(o.extract_field("spec")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
