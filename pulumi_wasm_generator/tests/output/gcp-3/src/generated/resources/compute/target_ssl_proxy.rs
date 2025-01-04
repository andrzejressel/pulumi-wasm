/// Represents a TargetSslProxy resource, which is used by one or more
/// global forwarding rule to route incoming SSL requests to a backend
/// service.
///
///
/// To get more information about TargetSslProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/targetSslProxies)
/// * How-to Guides
///     * [Setting Up SSL proxy for Google Cloud Load Balancing](https://cloud.google.com/compute/docs/load-balancing/tcp-ssl/)
///
/// ## Example Usage
///
/// ### Target Ssl Proxy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetSSLProxy
///     properties:
///       name: test-proxy
///       backendService: ${defaultBackendService.id}
///       sslCertificates:
///         - ${defaultSSLCertificate.id}
///   defaultSSLCertificate:
///     type: gcp:compute:SSLCertificate
///     name: default
///     properties:
///       name: default-cert
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/private.key
///           return: result
///       certificate:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/certificate.crt
///           return: result
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-service
///       protocol: SSL
///       healthChecks: ${defaultHealthCheck.id}
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '443'
/// ```
///
/// ## Import
///
/// TargetSslProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/targetSslProxies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetSslProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetSSLProxy:TargetSSLProxy default projects/{{project}}/global/targetSslProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetSSLProxy:TargetSSLProxy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetSSLProxy:TargetSSLProxy default {{name}}
/// ```
///
pub mod target_ssl_proxy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetSSLProxyArgs {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backend_service: pulumi_wasm_rust::Output<String>,
        /// A reference to the CertificateMap resource uri that identifies a certificate map
        /// associated with the given target proxy. This field can only be set for global target proxies.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}`.
        #[builder(into, default)]
        pub certificate_map: pulumi_wasm_rust::Output<Option<String>>,
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
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        #[builder(into, default)]
        pub proxy_header: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of SslCertificate resources that are used to authenticate
        /// connections between users and the load balancer. At least one
        /// SSL certificate must be specified.
        #[builder(into, default)]
        pub ssl_certificates: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A reference to the SslPolicy resource that will be associated with
        /// the TargetSslProxy resource. If not set, the TargetSslProxy
        /// resource will not have any SSL policy configured.
        #[builder(into, default)]
        pub ssl_policy: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetSSLProxyResult {
        /// A reference to the BackendService resource.
        ///
        ///
        /// - - -
        pub backend_service: pulumi_wasm_rust::Output<String>,
        /// A reference to the CertificateMap resource uri that identifies a certificate map
        /// associated with the given target proxy. This field can only be set for global target proxies.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}`.
        pub certificate_map: pulumi_wasm_rust::Output<Option<String>>,
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
        /// Specifies the type of proxy header to append before sending data to
        /// the backend.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `PROXY_V1`.
        pub proxy_header: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_wasm_rust::Output<i32>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// A list of SslCertificate resources that are used to authenticate
        /// connections between users and the load balancer. At least one
        /// SSL certificate must be specified.
        pub ssl_certificates: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A reference to the SslPolicy resource that will be associated with
        /// the TargetSslProxy resource. If not set, the TargetSslProxy
        /// resource will not have any SSL policy configured.
        pub ssl_policy: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TargetSSLProxyArgs) -> TargetSSLProxyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backend_service_binding = args.backend_service.get_inner();
        let certificate_map_binding = args.certificate_map.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let proxy_header_binding = args.proxy_header.get_inner();
        let ssl_certificates_binding = args.ssl_certificates.get_inner();
        let ssl_policy_binding = args.ssl_policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/targetSSLProxy:TargetSSLProxy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendService".into(),
                    value: &backend_service_binding,
                },
                register_interface::ObjectField {
                    name: "certificateMap".into(),
                    value: &certificate_map_binding,
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
                    name: "proxyHeader".into(),
                    value: &proxy_header_binding,
                },
                register_interface::ObjectField {
                    name: "sslCertificates".into(),
                    value: &ssl_certificates_binding,
                },
                register_interface::ObjectField {
                    name: "sslPolicy".into(),
                    value: &ssl_policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendService".into(),
                },
                register_interface::ResultField {
                    name: "certificateMap".into(),
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
                    name: "proxyHeader".into(),
                },
                register_interface::ResultField {
                    name: "proxyId".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "sslCertificates".into(),
                },
                register_interface::ResultField {
                    name: "sslPolicy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TargetSSLProxyResult {
            backend_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendService").unwrap(),
            ),
            certificate_map: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateMap").unwrap(),
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
            proxy_header: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyHeader").unwrap(),
            ),
            proxy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyId").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            ssl_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslCertificates").unwrap(),
            ),
            ssl_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslPolicy").unwrap(),
            ),
        }
    }
}
