/// ## Example Usage
///
/// ### Network Services Grpc Route Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:GrpcRoute
///     properties:
///       name: my-grpc-route
///       labels:
///         foo: bar
///       description: my description
///       hostnames:
///         - example
///       rules:
///         - matches:
///             - headers:
///                 - key: key
///                   value: value
///           action:
///             retryPolicy:
///               retryConditions:
///                 - cancelled
///               numRetries: 1
/// ```
/// ### Network Services Grpc Route Matches And Actions
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:GrpcRoute
///     properties:
///       name: my-grpc-route
///       labels:
///         foo: bar
///       description: my description
///       hostnames:
///         - example
///       rules:
///         - matches:
///             - headers:
///                 - key: key
///                   value: value
///             - headers:
///                 - key: key
///                   value: value
///               method:
///                 grpcService: foo
///                 grpcMethod: bar
///                 caseSensitive: true
///           action:
///             faultInjectionPolicy:
///               delay:
///                 fixedDelay: 1s
///                 percentage: 1
///               abort:
///                 httpStatus: 500
///                 percentage: 1
///             retryPolicy:
///               retryConditions:
///                 - cancelled
///               numRetries: 1
/// ```
/// ### Network Services Grpc Route Actions
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:GrpcRoute
///     properties:
///       name: my-grpc-route
///       labels:
///         foo: bar
///       description: my description
///       hostnames:
///         - example
///       rules:
///         - action:
///             faultInjectionPolicy:
///               delay:
///                 fixedDelay: 1s
///                 percentage: 1
///               abort:
///                 httpStatus: 500
///                 percentage: 1
///             retryPolicy:
///               retryConditions:
///                 - cancelled
///               numRetries: 1
/// ```
///
/// ## Import
///
/// GrpcRoute can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/grpcRoutes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, GrpcRoute can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/grpcRoute:GrpcRoute default projects/{{project}}/locations/global/grpcRoutes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/grpcRoute:GrpcRoute default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/grpcRoute:GrpcRoute default {{name}}
/// ```
///
pub mod grpc_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GrpcRouteArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway.
        #[builder(into, default)]
        pub gateways: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Required. Service hostnames with an optional port for which this route describes traffic.
        #[builder(into)]
        pub hostnames: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Set of label tags associated with the GrpcRoute resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh.
        #[builder(into, default)]
        pub meshes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the GrpcRoute resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rules that define how traffic is routed and handled.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::networkservices::GrpcRouteRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct GrpcRouteResult {
        /// Time the GrpcRoute was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway.
        pub gateways: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Required. Service hostnames with an optional port for which this route describes traffic.
        pub hostnames: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of label tags associated with the GrpcRoute resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh.
        pub meshes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the GrpcRoute resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Rules that define how traffic is routed and handled.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkservices::GrpcRouteRule>,
        >,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Time the GrpcRoute was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GrpcRouteArgs,
    ) -> GrpcRouteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let gateways_binding = args.gateways.get_output(context).get_inner();
        let hostnames_binding = args.hostnames.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let meshes_binding = args.meshes.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/grpcRoute:GrpcRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "gateways".into(),
                    value: &gateways_binding,
                },
                register_interface::ObjectField {
                    name: "hostnames".into(),
                    value: &hostnames_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "meshes".into(),
                    value: &meshes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GrpcRouteResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            gateways: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gateways"),
            ),
            hostnames: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostnames"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            meshes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("meshes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            rules: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rules")),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
