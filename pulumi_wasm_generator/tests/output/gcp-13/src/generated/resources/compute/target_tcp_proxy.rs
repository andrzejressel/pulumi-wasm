/// Represents a TargetTcpProxy resource, which is used by one or more
/// global forwarding rule to route incoming TCP requests to a Backend
/// service.
///
///
/// To get more information about TargetTcpProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/targetTcpProxies)
/// * How-to Guides
///     * [Setting Up TCP proxy for Google Cloud Load Balancing](https://cloud.google.com/compute/docs/load-balancing/tcp-ssl/tcp-proxy)
///
/// ## Example Usage
///
/// ### Target Tcp Proxy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetTCPProxy
///     properties:
///       name: test-proxy
///       backendService: ${defaultBackendService.id}
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-service
///       protocol: TCP
///       timeoutSec: 10
///       healthChecks: ${defaultHealthCheck.id}
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       tcpHealthCheck:
///         port: '443'
/// ```
///
/// ## Import
///
/// TargetTcpProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/targetTcpProxies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetTcpProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetTCPProxy:TargetTCPProxy default projects/{{project}}/global/targetTcpProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetTCPProxy:TargetTCPProxy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetTCPProxy:TargetTCPProxy default {{name}}
/// ```
///
pub mod target_tcp_proxy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetTCPProxyArgs {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backend_service: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        #[builder(into, default)]
        pub proxy_bind: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        #[builder(into, default)]
        pub proxy_header: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetTCPProxyResult {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        pub backend_service: pulumi_wasm_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        pub proxy_bind: pulumi_wasm_rust::Output<bool>,
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        pub proxy_header: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_wasm_rust::Output<i32>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TargetTCPProxyArgs) -> TargetTCPProxyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backend_service_binding = args.backend_service.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let proxy_bind_binding = args.proxy_bind.get_inner();
        let proxy_header_binding = args.proxy_header.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/targetTCPProxy:TargetTCPProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendService".into(),
                    value: &backend_service_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "proxyBind".into(),
                    value: &proxy_bind_binding,
                },
                register_interface::ObjectField {
                    name: "proxyHeader".into(),
                    value: &proxy_header_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendService".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "proxyBind".into(),
                },
                register_interface::ResultField {
                    name: "proxyHeader".into(),
                },
                register_interface::ResultField {
                    name: "proxyId".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TargetTCPProxyResult {
            backend_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendService").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            proxy_bind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyBind").unwrap(),
            ),
            proxy_header: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyHeader").unwrap(),
            ),
            proxy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyId").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
        }
    }
}
