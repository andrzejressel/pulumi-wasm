/// ## Example Usage
///
/// ### Network Services Http Route Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:HttpRoute
///     properties:
///       name: my-http-route
///       labels:
///         foo: bar
///       description: my description
///       hostnames:
///         - example
///       rules:
///         - matches:
///             - queryParameters:
///                 - queryParameter: key
///                   exactMatch: value
///               fullPathMatch: example
/// ```
/// ### Network Services Http Route Matches And Actions
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:HttpRoute
///     properties:
///       name: my-http-route
///       labels:
///         foo: bar
///       description: my description
///       hostnames:
///         - example
///       rules:
///         - matches:
///             - headers:
///                 - header: header
///                   invertMatch: false
///                   regexMatch: header-value
///               queryParameters:
///                 - queryParameter: key
///                   exactMatch: value
///               prefixMatch: example
///               ignoreCase: false
///             - headers:
///                 - header: header
///                   invertMatch: false
///                   presentMatch: true
///               queryParameters:
///                 - queryParameter: key
///                   regexMatch: value
///               regexMatch: example
///               ignoreCase: false
///             - headers:
///                 - header: header
///                   invertMatch: false
///                   presentMatch: true
///               queryParameters:
///                 - queryParameter: key
///                   presentMatch: true
///               fullPathMatch: example
///               ignoreCase: false
///           action:
///             redirect:
///               hostRedirect: new-host
///               pathRedirect: new-path
///               prefixRewrite: new-prefix
///               httpsRedirect: true
///               stripQuery: true
///               portRedirect: 8081
///             urlRewrite:
///               pathPrefixRewrite: new-prefix
///               hostRewrite: new-host
///             retryPolicy:
///               retryConditions:
///                 - server_error
///               numRetries: 1
///               perTryTimeout: 1s
///             requestMirrorPolicy:
///               destination:
///                 serviceName: new
///                 weight: 1
///             corsPolicy:
///               allowOrigins:
///                 - example
///               allowMethods:
///                 - GET
///                 - PUT
///               allowHeaders:
///                 - version
///                 - type
///               exposeHeaders:
///                 - version
///                 - type
///               maxAge: 1s
///               allowCredentials: true
///               disabled: false
/// ```
/// ### Network Services Http Route Actions
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:HttpRoute
///     properties:
///       name: my-http-route
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
///             urlRewrite:
///               pathPrefixRewrite: new-prefix
///               hostRewrite: new-host
///             retryPolicy:
///               retryConditions:
///                 - server_error
///               numRetries: 1
///               perTryTimeout: 1s
///             requestMirrorPolicy:
///               destination:
///                 serviceName: new
///                 weight: 1
///             corsPolicy:
///               allowOrigins:
///                 - example
///               allowMethods:
///                 - GET
///                 - PUT
///               allowHeaders:
///                 - version
///                 - type
///               exposeHeaders:
///                 - version
///                 - type
///               maxAge: 1s
///               allowCredentials: true
///               disabled: false
///             requestHeaderModifier:
///               set:
///                 version: '1'
///                 type: json
///               add:
///                 minor-version: '1'
///               removes:
///                 - arg
///             responseHeaderModifier:
///               set:
///                 version: '1'
///                 type: json
///               add:
///                 minor-version: '1'
///               removes:
///                 - removearg
/// ```
/// ### Network Services Http Route Mesh Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:Mesh
///     properties:
///       name: my-http-route
///       labels:
///         foo: bar
///       description: my description
///   defaultHttpRoute:
///     type: gcp:networkservices:HttpRoute
///     name: default
///     properties:
///       name: my-http-route
///       labels:
///         foo: bar
///       description: my description
///       hostnames:
///         - example
///       meshes:
///         - ${default.id}
///       rules:
///         - matches:
///             - queryParameters:
///                 - queryParameter: key
///                   exactMatch: value
///               fullPathMatch: example
/// ```
///
/// ## Import
///
/// HttpRoute can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/httpRoutes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, HttpRoute can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/httpRoute:HttpRoute default projects/{{project}}/locations/global/httpRoutes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/httpRoute:HttpRoute default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/httpRoute:HttpRoute default {{name}}
/// ```
///
pub mod http_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HttpRouteArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Gateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests
        /// served by the gateway. Each gateway reference should match the pattern:
        /// projects/*/locations/global/gateways/<gateway_name>
        #[builder(into, default)]
        pub gateways: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Set of hosts that should match against the HTTP host header to select a HttpRoute to process the request.
        #[builder(into)]
        pub hostnames: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Set of label tags associated with the HttpRoute resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Meshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served
        /// by the mesh. Each mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>. The attached
        /// Mesh should be of a type SIDECAR.
        #[builder(into, default)]
        pub meshes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the HttpRoute resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Rules that define how traffic is routed and handled.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::networkservices::HttpRouteRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct HttpRouteResult {
        /// Time the HttpRoute was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Gateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests
        /// served by the gateway. Each gateway reference should match the pattern:
        /// projects/*/locations/global/gateways/<gateway_name>
        pub gateways: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Set of hosts that should match against the HTTP host header to select a HttpRoute to process the request.
        pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of label tags associated with the HttpRoute resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Meshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served
        /// by the mesh. Each mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>. The attached
        /// Mesh should be of a type SIDECAR.
        pub meshes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the HttpRoute resource.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Rules that define how traffic is routed and handled.
        /// Structure is documented below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::networkservices::HttpRouteRule>,
        >,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Time the HttpRoute was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HttpRouteArgs,
    ) -> HttpRouteResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            type_: "gcp:networkservices/httpRoute:HttpRoute".into(),
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
        HttpRouteResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            gateways: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gateways"),
            ),
            hostnames: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostnames"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            meshes: pulumi_wasm_rust::__private::into_domain(o.extract_field("meshes")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
