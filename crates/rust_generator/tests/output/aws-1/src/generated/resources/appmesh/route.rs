/// Provides an AWS App Mesh route resource.
///
/// ## Example Usage
///
/// ### HTTP Routing
///
/// ```yaml
/// resources:
///   serviceb:
///     type: aws:appmesh:Route
///     properties:
///       name: serviceB-route
///       meshName: ${simple.id}
///       virtualRouterName: ${servicebAwsAppmeshVirtualRouter.name}
///       spec:
///         httpRoute:
///           match:
///             prefix: /
///           action:
///             weightedTargets:
///               - virtualNode: ${serviceb1.name}
///                 weight: 90
///               - virtualNode: ${serviceb2.name}
///                 weight: 10
/// ```
///
/// ### HTTP Header Routing
///
/// ```yaml
/// resources:
///   serviceb:
///     type: aws:appmesh:Route
///     properties:
///       name: serviceB-route
///       meshName: ${simple.id}
///       virtualRouterName: ${servicebAwsAppmeshVirtualRouter.name}
///       spec:
///         httpRoute:
///           match:
///             method: POST
///             prefix: /
///             scheme: https
///             headers:
///               - name: clientRequestId
///                 match:
///                   prefix: '123'
///           action:
///             weightedTargets:
///               - virtualNode: ${servicebAwsAppmeshVirtualNode.name}
///                 weight: 100
/// ```
///
/// ### Retry Policy
///
/// ```yaml
/// resources:
///   serviceb:
///     type: aws:appmesh:Route
///     properties:
///       name: serviceB-route
///       meshName: ${simple.id}
///       virtualRouterName: ${servicebAwsAppmeshVirtualRouter.name}
///       spec:
///         httpRoute:
///           match:
///             prefix: /
///           retryPolicy:
///             httpRetryEvents:
///               - server-error
///             maxRetries: 1
///             perRetryTimeout:
///               unit: s
///               value: 15
///           action:
///             weightedTargets:
///               - virtualNode: ${servicebAwsAppmeshVirtualNode.name}
///                 weight: 100
/// ```
///
/// ### TCP Routing
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serviceb = route::create(
///         "serviceb",
///         RouteArgs::builder()
///             .mesh_name("${simple.id}")
///             .name("serviceB-route")
///             .spec(
///                 RouteSpec::builder()
///                     .tcpRoute(
///                         RouteSpecTcpRoute::builder()
///                             .action(
///                                 RouteSpecTcpRouteAction::builder()
///                                     .weightedTargets(
///                                         vec![
///                                             RouteSpecTcpRouteActionWeightedTarget::builder()
///                                             .virtualNode("${serviceb1.name}").weight(100)
///                                             .build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .virtual_router_name("${servicebAwsAppmeshVirtualRouter.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Mesh virtual routes using `mesh_name` and `virtual_router_name` together with the route's `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appmesh/route:Route serviceb simpleapp/serviceB/serviceB-route
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// Name of the service mesh in which to create the route. Must be between 1 and 255 characters in length.
        #[builder(into)]
        pub mesh_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        #[builder(into, default)]
        pub mesh_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name to use for the route. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Route specification to apply.
        #[builder(into)]
        pub spec: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appmesh::RouteSpec,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the virtual router in which to create the route. Must be between 1 and 255 characters in length.
        #[builder(into)]
        pub virtual_router_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// ARN of the route.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the route.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Last update date of the route.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Name of the service mesh in which to create the route. Must be between 1 and 255 characters in length.
        pub mesh_name: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        pub mesh_owner: pulumi_gestalt_rust::Output<String>,
        /// Name to use for the route. Must be between 1 and 255 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// Route specification to apply.
        pub spec: pulumi_gestalt_rust::Output<super::super::types::appmesh::RouteSpec>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of the virtual router in which to create the route. Must be between 1 and 255 characters in length.
        pub virtual_router_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RouteArgs,
    ) -> RouteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mesh_name_binding = args.mesh_name.get_output(context).get_inner();
        let mesh_owner_binding = args.mesh_owner.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let spec_binding = args.spec.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let virtual_router_name_binding = args
            .virtual_router_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appmesh/route:Route".into(),
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
                register_interface::ObjectField {
                    name: "virtualRouterName".into(),
                    value: &virtual_router_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouteResult {
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
            virtual_router_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualRouterName"),
            ),
        }
    }
}
