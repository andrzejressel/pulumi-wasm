/// Health Checks determine whether instances are responsive and able to do work.
/// They are an important part of a comprehensive load balancing configuration,
/// as they enable monitoring instances behind load balancers.
///
/// Health Checks poll instances at a specified interval. Instances that
/// do not respond successfully to some number of probes in a row are marked
/// as unhealthy. No new connections are sent to unhealthy instances,
/// though existing connections will continue. The health check will
/// continue to poll unhealthy instances. If an instance later responds
/// successfully to some number of consecutive probes, it is marked
/// healthy again and can receive new connections.
///
/// ~>**NOTE**: Legacy HTTP(S) health checks must be used for target pool-based network
/// load balancers. See the [official guide](https://cloud.google.com/load-balancing/docs/health-check-concepts#selecting_hc)
/// for choosing a type of health check.
///
///
/// To get more information about HealthCheck, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/healthChecks)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/health-checks)
///
/// ## Example Usage
///
/// ### Health Check Tcp
///
///
/// ```yaml
/// resources:
///   tcp-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: tcp-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       tcpHealthCheck:
///         port: '80'
/// ```
/// ### Health Check Tcp Full
///
///
/// ```yaml
/// resources:
///   tcp-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: tcp-health-check
///       description: Health check via tcp
///       timeoutSec: 1
///       checkIntervalSec: 1
///       healthyThreshold: 4
///       unhealthyThreshold: 5
///       tcpHealthCheck:
///         portName: health-check-port
///         portSpecification: USE_NAMED_PORT
///         request: ARE YOU HEALTHY?
///         proxyHeader: NONE
///         response: I AM HEALTHY
/// ```
/// ### Health Check Ssl
///
///
/// ```yaml
/// resources:
///   ssl-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: ssl-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       sslHealthCheck:
///         port: '443'
/// ```
/// ### Health Check Ssl Full
///
///
/// ```yaml
/// resources:
///   ssl-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: ssl-health-check
///       description: Health check via ssl
///       timeoutSec: 1
///       checkIntervalSec: 1
///       healthyThreshold: 4
///       unhealthyThreshold: 5
///       sslHealthCheck:
///         portName: health-check-port
///         portSpecification: USE_NAMED_PORT
///         request: ARE YOU HEALTHY?
///         proxyHeader: NONE
///         response: I AM HEALTHY
/// ```
/// ### Health Check Http
///
///
/// ```yaml
/// resources:
///   http-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: http-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       httpHealthCheck:
///         port: 80
/// ```
/// ### Health Check Http Full
///
///
/// ```yaml
/// resources:
///   http-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: http-health-check
///       description: Health check via http
///       timeoutSec: 1
///       checkIntervalSec: 1
///       healthyThreshold: 4
///       unhealthyThreshold: 5
///       httpHealthCheck:
///         portName: health-check-port
///         portSpecification: USE_NAMED_PORT
///         host: 1.2.3.4
///         requestPath: /mypath
///         proxyHeader: NONE
///         response: I AM HEALTHY
/// ```
/// ### Health Check Https
///
///
/// ```yaml
/// resources:
///   https-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: https-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       httpsHealthCheck:
///         port: '443'
/// ```
/// ### Health Check Https Full
///
///
/// ```yaml
/// resources:
///   https-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: https-health-check
///       description: Health check via https
///       timeoutSec: 1
///       checkIntervalSec: 1
///       healthyThreshold: 4
///       unhealthyThreshold: 5
///       httpsHealthCheck:
///         portName: health-check-port
///         portSpecification: USE_NAMED_PORT
///         host: 1.2.3.4
///         requestPath: /mypath
///         proxyHeader: NONE
///         response: I AM HEALTHY
/// ```
/// ### Health Check Http2
///
///
/// ```yaml
/// resources:
///   http2-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: http2-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       http2HealthCheck:
///         port: '443'
/// ```
/// ### Health Check Http2 Full
///
///
/// ```yaml
/// resources:
///   http2-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: http2-health-check
///       description: Health check via http2
///       timeoutSec: 1
///       checkIntervalSec: 1
///       healthyThreshold: 4
///       unhealthyThreshold: 5
///       http2HealthCheck:
///         portName: health-check-port
///         portSpecification: USE_NAMED_PORT
///         host: 1.2.3.4
///         requestPath: /mypath
///         proxyHeader: NONE
///         response: I AM HEALTHY
/// ```
/// ### Health Check Grpc
///
///
/// ```yaml
/// resources:
///   grpc-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: grpc-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       grpcHealthCheck:
///         port: '443'
/// ```
/// ### Health Check Grpc Full
///
///
/// ```yaml
/// resources:
///   grpc-health-check:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: grpc-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       grpcHealthCheck:
///         portName: health-check-port
///         portSpecification: USE_NAMED_PORT
///         grpcServiceName: testservice
/// ```
/// ### Health Check With Logging
///
///
/// ```yaml
/// resources:
///   health-check-with-logging:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: tcp-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       tcpHealthCheck:
///         port: '22'
///       logConfig:
///         enable: true
/// ```
/// ### Compute Health Check Http Source Regions
///
///
/// ```yaml
/// resources:
///   http-health-check-with-source-regions:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: http-health-check
///       checkIntervalSec: 30
///       httpHealthCheck:
///         port: 80
///         portSpecification: USE_FIXED_PORT
///       sourceRegions:
///         - us-west1
///         - us-central1
///         - us-east5
/// ```
/// ### Compute Health Check Https Source Regions
///
///
/// ```yaml
/// resources:
///   https-health-check-with-source-regions:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: https-health-check
///       checkIntervalSec: 30
///       httpsHealthCheck:
///         port: 80
///         portSpecification: USE_FIXED_PORT
///       sourceRegions:
///         - us-west1
///         - us-central1
///         - us-east5
/// ```
/// ### Compute Health Check Tcp Source Regions
///
///
/// ```yaml
/// resources:
///   tcp-health-check-with-source-regions:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: tcp-health-check
///       checkIntervalSec: 30
///       tcpHealthCheck:
///         port: 80
///         portSpecification: USE_FIXED_PORT
///       sourceRegions:
///         - us-west1
///         - us-central1
///         - us-east5
/// ```
///
/// ## Import
///
/// HealthCheck can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/healthChecks/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, HealthCheck can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/healthCheck:HealthCheck default projects/{{project}}/global/healthChecks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/healthCheck:HealthCheck default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/healthCheck:HealthCheck default {{name}}
/// ```
///
pub mod health_check {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HealthCheckArgs {
        /// How often (in seconds) to send a health check. The default value is 5
        /// seconds.
        #[builder(into, default)]
        pub check_interval_sec: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub grpc_health_check: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::HealthCheckGrpcHealthCheck>,
        >,
        /// A so-far unhealthy instance will be marked healthy after this many
        /// consecutive successes. The default value is 2.
        #[builder(into, default)]
        pub healthy_threshold: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub http2_health_check: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::HealthCheckHttp2HealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub http_health_check: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::HealthCheckHttpHealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub https_health_check: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::HealthCheckHttpsHealthCheck>,
        >,
        /// Configure logging on this health check.
        /// Structure is documented below.
        #[builder(into, default)]
        pub log_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::HealthCheckLogConfig>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the
        /// last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The list of cloud regions from which health checks are performed. If
        /// any regions are specified, then exactly 3 regions should be specified.
        /// The region names must be valid names of Google Cloud regions. This can
        /// only be set for global health check. If this list is non-empty, then
        /// there are restrictions on what other health check fields are supported
        /// and what other resources can use this health check:
        /// * SSL, HTTP2, and GRPC protocols are not supported.
        /// * The TCP request field is not supported.
        /// * The proxyHeader field for HTTP, HTTPS, and TCP is not supported.
        /// * The checkIntervalSec field must be at least 30.
        /// * The health check cannot be used with BackendService nor with managed
        /// instance group auto-healing.
        #[builder(into, default)]
        pub source_regions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ssl_health_check: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::HealthCheckSslHealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub tcp_health_check: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::HealthCheckTcpHealthCheck>,
        >,
        /// How long (in seconds) to wait before claiming failure.
        /// The default value is 5 seconds.  It is invalid for timeoutSec to have
        /// greater value than checkIntervalSec.
        #[builder(into, default)]
        pub timeout_sec: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A so-far healthy instance will be marked unhealthy after this many
        /// consecutive failures. The default value is 2.
        #[builder(into, default)]
        pub unhealthy_threshold: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct HealthCheckResult {
        /// How often (in seconds) to send a health check. The default value is 5
        /// seconds.
        pub check_interval_sec: pulumi_wasm_rust::Output<Option<i32>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        pub grpc_health_check: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::HealthCheckGrpcHealthCheck>,
        >,
        /// A so-far unhealthy instance will be marked healthy after this many
        /// consecutive successes. The default value is 2.
        pub healthy_threshold: pulumi_wasm_rust::Output<Option<i32>>,
        /// A nested object resource.
        /// Structure is documented below.
        pub http2_health_check: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::HealthCheckHttp2HealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        pub http_health_check: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::HealthCheckHttpHealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        pub https_health_check: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::HealthCheckHttpsHealthCheck>,
        >,
        /// Configure logging on this health check.
        /// Structure is documented below.
        pub log_config: pulumi_wasm_rust::Output<
            super::super::types::compute::HealthCheckLogConfig,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the
        /// last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The list of cloud regions from which health checks are performed. If
        /// any regions are specified, then exactly 3 regions should be specified.
        /// The region names must be valid names of Google Cloud regions. This can
        /// only be set for global health check. If this list is non-empty, then
        /// there are restrictions on what other health check fields are supported
        /// and what other resources can use this health check:
        /// * SSL, HTTP2, and GRPC protocols are not supported.
        /// * The TCP request field is not supported.
        /// * The proxyHeader field for HTTP, HTTPS, and TCP is not supported.
        /// * The checkIntervalSec field must be at least 30.
        /// * The health check cannot be used with BackendService nor with managed
        /// instance group auto-healing.
        pub source_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A nested object resource.
        /// Structure is documented below.
        pub ssl_health_check: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::HealthCheckSslHealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        pub tcp_health_check: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::HealthCheckTcpHealthCheck>,
        >,
        /// How long (in seconds) to wait before claiming failure.
        /// The default value is 5 seconds.  It is invalid for timeoutSec to have
        /// greater value than checkIntervalSec.
        pub timeout_sec: pulumi_wasm_rust::Output<Option<i32>>,
        /// The type of the health check. One of HTTP, HTTPS, TCP, or SSL.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// A so-far healthy instance will be marked unhealthy after this many
        /// consecutive failures. The default value is 2.
        pub unhealthy_threshold: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HealthCheckArgs,
    ) -> HealthCheckResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let check_interval_sec_binding = args
            .check_interval_sec
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let grpc_health_check_binding = args
            .grpc_health_check
            .get_output(context)
            .get_inner();
        let healthy_threshold_binding = args
            .healthy_threshold
            .get_output(context)
            .get_inner();
        let http2_health_check_binding = args
            .http2_health_check
            .get_output(context)
            .get_inner();
        let http_health_check_binding = args
            .http_health_check
            .get_output(context)
            .get_inner();
        let https_health_check_binding = args
            .https_health_check
            .get_output(context)
            .get_inner();
        let log_config_binding = args.log_config.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let source_regions_binding = args.source_regions.get_output(context).get_inner();
        let ssl_health_check_binding = args
            .ssl_health_check
            .get_output(context)
            .get_inner();
        let tcp_health_check_binding = args
            .tcp_health_check
            .get_output(context)
            .get_inner();
        let timeout_sec_binding = args.timeout_sec.get_output(context).get_inner();
        let unhealthy_threshold_binding = args
            .unhealthy_threshold
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/healthCheck:HealthCheck".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "checkIntervalSec".into(),
                    value: &check_interval_sec_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "grpcHealthCheck".into(),
                    value: &grpc_health_check_binding,
                },
                register_interface::ObjectField {
                    name: "healthyThreshold".into(),
                    value: &healthy_threshold_binding,
                },
                register_interface::ObjectField {
                    name: "http2HealthCheck".into(),
                    value: &http2_health_check_binding,
                },
                register_interface::ObjectField {
                    name: "httpHealthCheck".into(),
                    value: &http_health_check_binding,
                },
                register_interface::ObjectField {
                    name: "httpsHealthCheck".into(),
                    value: &https_health_check_binding,
                },
                register_interface::ObjectField {
                    name: "logConfig".into(),
                    value: &log_config_binding,
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
                    name: "sourceRegions".into(),
                    value: &source_regions_binding,
                },
                register_interface::ObjectField {
                    name: "sslHealthCheck".into(),
                    value: &ssl_health_check_binding,
                },
                register_interface::ObjectField {
                    name: "tcpHealthCheck".into(),
                    value: &tcp_health_check_binding,
                },
                register_interface::ObjectField {
                    name: "timeoutSec".into(),
                    value: &timeout_sec_binding,
                },
                register_interface::ObjectField {
                    name: "unhealthyThreshold".into(),
                    value: &unhealthy_threshold_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "checkIntervalSec".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "grpcHealthCheck".into(),
                },
                register_interface::ResultField {
                    name: "healthyThreshold".into(),
                },
                register_interface::ResultField {
                    name: "http2HealthCheck".into(),
                },
                register_interface::ResultField {
                    name: "httpHealthCheck".into(),
                },
                register_interface::ResultField {
                    name: "httpsHealthCheck".into(),
                },
                register_interface::ResultField {
                    name: "logConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "sourceRegions".into(),
                },
                register_interface::ResultField {
                    name: "sslHealthCheck".into(),
                },
                register_interface::ResultField {
                    name: "tcpHealthCheck".into(),
                },
                register_interface::ResultField {
                    name: "timeoutSec".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "unhealthyThreshold".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HealthCheckResult {
            check_interval_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checkIntervalSec").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            grpc_health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grpcHealthCheck").unwrap(),
            ),
            healthy_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthyThreshold").unwrap(),
            ),
            http2_health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("http2HealthCheck").unwrap(),
            ),
            http_health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpHealthCheck").unwrap(),
            ),
            https_health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsHealthCheck").unwrap(),
            ),
            log_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            source_regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceRegions").unwrap(),
            ),
            ssl_health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslHealthCheck").unwrap(),
            ),
            tcp_health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tcpHealthCheck").unwrap(),
            ),
            timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeoutSec").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            unhealthy_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("unhealthyThreshold").unwrap(),
            ),
        }
    }
}
