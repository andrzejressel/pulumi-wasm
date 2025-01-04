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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// Name of the service mesh in which to create the route. Must be between 1 and 255 characters in length.
        #[builder(into)]
        pub mesh_name: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        #[builder(into, default)]
        pub mesh_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Name to use for the route. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Route specification to apply.
        #[builder(into)]
        pub spec: pulumi_wasm_rust::Output<super::super::types::appmesh::RouteSpec>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the virtual router in which to create the route. Must be between 1 and 255 characters in length.
        #[builder(into)]
        pub virtual_router_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// ARN of the route.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Creation date of the route.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Last update date of the route.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Name of the service mesh in which to create the route. Must be between 1 and 255 characters in length.
        pub mesh_name: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the service mesh's owner. Defaults to the account ID the AWS provider is currently connected to.
        pub mesh_owner: pulumi_wasm_rust::Output<String>,
        /// Name to use for the route. Must be between 1 and 255 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        /// Route specification to apply.
        pub spec: pulumi_wasm_rust::Output<super::super::types::appmesh::RouteSpec>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of the virtual router in which to create the route. Must be between 1 and 255 characters in length.
        pub virtual_router_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RouteArgs) -> RouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mesh_name_binding = args.mesh_name.get_inner();
        let mesh_owner_binding = args.mesh_owner.get_inner();
        let name_binding = args.name.get_inner();
        let spec_binding = args.spec.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_router_name_binding = args.virtual_router_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appmesh/route:Route".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "meshName".into(),
                },
                register_interface::ResultField {
                    name: "meshOwner".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceOwner".into(),
                },
                register_interface::ResultField {
                    name: "spec".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "virtualRouterName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouteResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            mesh_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("meshName").unwrap(),
            ),
            mesh_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("meshOwner").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceOwner").unwrap(),
            ),
            spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spec").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            virtual_router_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualRouterName").unwrap(),
            ),
        }
    }
}
