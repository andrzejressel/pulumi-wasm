/// ## Example Usage
///
/// ### Network Services Tls Route Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("my-backend-service")
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("backend-service-health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let defaultTlsRoute = tls_route::create(
///         "defaultTlsRoute",
///         TlsRouteArgs::builder()
///             .description("my description")
///             .name("my-tls-route")
///             .rules(
///                 vec![
///                     TlsRouteRule::builder().action(TlsRouteRuleAction::builder()
///                     .destinations(vec![TlsRouteRuleActionDestination::builder()
///                     .serviceName("${default.id}").weight(1).build_struct(),])
///                     .build_struct()).matches(vec![TlsRouteRuleMatch::builder()
///                     .alpns(vec!["http/1.1",]).sniHosts(vec!["example.com",])
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Services Tls Route Mesh Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: my-backend-service
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: backend-service-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   defaultMesh:
///     type: gcp:networkservices:Mesh
///     name: default
///     properties:
///       name: my-tls-route
///       labels:
///         foo: bar
///       description: my description
///   defaultTlsRoute:
///     type: gcp:networkservices:TlsRoute
///     name: default
///     properties:
///       name: my-tls-route
///       description: my description
///       meshes:
///         - ${defaultMesh.id}
///       rules:
///         - matches:
///             - sniHosts:
///                 - example.com
///               alpns:
///                 - http/1.1
///           action:
///             destinations:
///               - serviceName: ${default.id}
///                 weight: 1
/// ```
/// ### Network Services Tls Route Gateway Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: my-backend-service
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: backend-service-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   defaultGateway:
///     type: gcp:networkservices:Gateway
///     name: default
///     properties:
///       name: my-tls-route
///       labels:
///         foo: bar
///       description: my description
///       scope: my-scope
///       type: OPEN_MESH
///       ports:
///         - 443
///   defaultTlsRoute:
///     type: gcp:networkservices:TlsRoute
///     name: default
///     properties:
///       name: my-tls-route
///       description: my description
///       gateways:
///         - ${defaultGateway.id}
///       rules:
///         - matches:
///             - sniHosts:
///                 - example.com
///               alpns:
///                 - http/1.1
///           action:
///             destinations:
///               - serviceName: ${default.id}
///                 weight: 1
/// ```
///
/// ## Import
///
/// TlsRoute can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/tlsRoutes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TlsRoute can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/tlsRoute:TlsRoute default projects/{{project}}/locations/global/tlsRoutes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/tlsRoute:TlsRoute default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/tlsRoute:TlsRoute default {{name}}
/// ```
///
pub mod tls_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TlsRouteArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests
        /// served by the gateway. Each gateway reference should match the pattern:
        /// projects/*/locations/global/gateways/<gateway_name>
        #[builder(into, default)]
        pub gateways: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served
        /// by the mesh. Each mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name> The attached
        /// Mesh should be of a type SIDECAR
        #[builder(into, default)]
        pub meshes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the TlsRoute resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Rules that define how traffic is routed and handled.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::networkservices::TlsRouteRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct TlsRouteResult {
        /// Time the TlsRoute was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests
        /// served by the gateway. Each gateway reference should match the pattern:
        /// projects/*/locations/global/gateways/<gateway_name>
        pub gateways: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served
        /// by the mesh. Each mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name> The attached
        /// Mesh should be of a type SIDECAR
        pub meshes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the TlsRoute resource.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Rules that define how traffic is routed and handled.
        /// Structure is documented below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::networkservices::TlsRouteRule>,
        >,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Time the TlsRoute was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TlsRouteArgs) -> TlsRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let gateways_binding = args.gateways.get_inner();
        let meshes_binding = args.meshes.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let rules_binding = args.rules.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/tlsRoute:TlsRoute".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "gateways".into(),
                },
                register_interface::ResultField {
                    name: "meshes".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TlsRouteResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            gateways: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gateways").unwrap(),
            ),
            meshes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("meshes").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
