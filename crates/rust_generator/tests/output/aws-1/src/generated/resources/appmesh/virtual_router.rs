/// Provides an AWS App Mesh virtual router resource.
///
/// ## Breaking Changes
///
/// Because of backward incompatible API changes (read [here](https://github.com/awslabs/aws-app-mesh-examples/issues/92) and [here](https://github.com/awslabs/aws-app-mesh-examples/issues/94)), `aws.appmesh.VirtualRouter` resource definitions created with provider versions earlier than v2.3.0 will need to be modified:
///
/// * Remove service `service_names` from the `spec` argument. AWS has created a `aws.appmesh.VirtualService` resource for each service name. Import these resource using `pulumi import`.
///
/// * Add a `listener` configuration block to the `spec` argument.
///
/// The state associated with existing resources will automatically be migrated.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serviceb = virtual_router::create(
///         "serviceb",
///         VirtualRouterArgs::builder()
///             .mesh_name("${simple.id}")
///             .name("serviceB")
///             .spec(
///                 VirtualRouterSpec::builder()
///                     .listeners(
///                         vec![
///                             VirtualRouterSpecListener::builder()
///                             .portMapping(VirtualRouterSpecListenerPortMapping::builder()
///                             .port(8080).protocol("http").build_struct()).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Mesh virtual routers using `mesh_name` together with the virtual router's `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appmesh/virtualRouter:VirtualRouter serviceb simpleapp/serviceB
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_router {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualRouterArgs {
        /// Name of the service mesh in which to create the virtual router. Must be between 1 and 255 characters in length.
        #[builder(into)]
        pub mesh_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        #[builder(into, default)]
        pub mesh_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name to use for the virtual router. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Virtual router specification to apply.
        #[builder(into)]
        pub spec: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appmesh::VirtualRouterSpec,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualRouterResult {
        /// ARN of the virtual router.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the virtual router.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Last update date of the virtual router.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Name of the service mesh in which to create the virtual router. Must be between 1 and 255 characters in length.
        pub mesh_name: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        pub mesh_owner: pulumi_gestalt_rust::Output<String>,
        /// Name to use for the virtual router. Must be between 1 and 255 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// Virtual router specification to apply.
        pub spec: pulumi_gestalt_rust::Output<
            super::super::types::appmesh::VirtualRouterSpec,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VirtualRouterArgs,
    ) -> VirtualRouterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mesh_name_binding = args.mesh_name.get_output(context).get_inner();
        let mesh_owner_binding = args.mesh_owner.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let spec_binding = args.spec.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appmesh/virtualRouter:VirtualRouter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "meshName".into(),
                    value: &mesh_name_binding,
                },
                register_interface::ObjectField {
                    name: "meshOwner".into(),
                    value: &mesh_owner_binding,
                },
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
        VirtualRouterResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
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
            spec: pulumi_gestalt_rust::__private::into_domain(o.extract_field("spec")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
