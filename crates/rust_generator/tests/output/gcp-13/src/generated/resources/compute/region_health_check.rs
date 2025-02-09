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
///
/// To get more information about RegionHealthCheck, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionHealthChecks)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/health-checks)
///
/// ## Example Usage
///
/// ### Region Health Check Tcp
///
///
/// ```yaml
/// resources:
///   tcp-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: tcp-region-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       tcpHealthCheck:
///         port: '80'
/// ```
/// ### Region Health Check Tcp Full
///
///
/// ```yaml
/// resources:
///   tcp-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: tcp-region-health-check
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
/// ### Region Health Check Ssl
///
///
/// ```yaml
/// resources:
///   ssl-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: ssl-region-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       sslHealthCheck:
///         port: '443'
/// ```
/// ### Region Health Check Ssl Full
///
///
/// ```yaml
/// resources:
///   ssl-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: ssl-region-health-check
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
/// ### Region Health Check Http
///
///
/// ```yaml
/// resources:
///   http-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: http-region-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       httpHealthCheck:
///         port: '80'
/// ```
/// ### Region Health Check Http Logs
///
///
/// ```yaml
/// resources:
///   http-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: http-region-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       httpHealthCheck:
///         port: '80'
///       logConfig:
///         enable: true
/// ```
/// ### Region Health Check Http Full
///
///
/// ```yaml
/// resources:
///   http-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: http-region-health-check
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
/// ### Region Health Check Https
///
///
/// ```yaml
/// resources:
///   https-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: https-region-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       httpsHealthCheck:
///         port: '443'
/// ```
/// ### Region Health Check Https Full
///
///
/// ```yaml
/// resources:
///   https-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: https-region-health-check
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
/// ### Region Health Check Http2
///
///
/// ```yaml
/// resources:
///   http2-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: http2-region-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       http2HealthCheck:
///         port: '443'
/// ```
/// ### Region Health Check Http2 Full
///
///
/// ```yaml
/// resources:
///   http2-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: http2-region-health-check
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
/// ### Region Health Check Grpc
///
///
/// ```yaml
/// resources:
///   grpc-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: grpc-region-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       grpcHealthCheck:
///         port: '443'
/// ```
/// ### Region Health Check Grpc Full
///
///
/// ```yaml
/// resources:
///   grpc-region-health-check:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: grpc-region-health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       grpcHealthCheck:
///         portName: health-check-port
///         portSpecification: USE_NAMED_PORT
///         grpcServiceName: testservice
/// ```
///
/// ## Import
///
/// RegionHealthCheck can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/healthChecks/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionHealthCheck can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionHealthCheck:RegionHealthCheck default projects/{{project}}/regions/{{region}}/healthChecks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionHealthCheck:RegionHealthCheck default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionHealthCheck:RegionHealthCheck default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionHealthCheck:RegionHealthCheck default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_health_check {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionHealthCheckArgs {
        /// How often (in seconds) to send a health check. The default value is 5
        /// seconds.
        #[builder(into, default)]
        pub check_interval_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub grpc_health_check: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionHealthCheckGrpcHealthCheck>,
        >,
        /// A so-far unhealthy instance will be marked healthy after this many
        /// consecutive successes. The default value is 2.
        #[builder(into, default)]
        pub healthy_threshold: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub http2_health_check: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionHealthCheckHttp2HealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub http_health_check: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionHealthCheckHttpHealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub https_health_check: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionHealthCheckHttpsHealthCheck>,
        >,
        /// Configure logging on this health check.
        /// Structure is documented below.
        #[builder(into, default)]
        pub log_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionHealthCheckLogConfig>,
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
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Region in which the created health check should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ssl_health_check: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionHealthCheckSslHealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub tcp_health_check: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionHealthCheckTcpHealthCheck>,
        >,
        /// How long (in seconds) to wait before claiming failure.
        /// The default value is 5 seconds.  It is invalid for timeoutSec to have
        /// greater value than checkIntervalSec.
        #[builder(into, default)]
        pub timeout_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A so-far healthy instance will be marked unhealthy after this many
        /// consecutive failures. The default value is 2.
        #[builder(into, default)]
        pub unhealthy_threshold: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct RegionHealthCheckResult {
        /// How often (in seconds) to send a health check. The default value is 5
        /// seconds.
        pub check_interval_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        pub grpc_health_check: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionHealthCheckGrpcHealthCheck>,
        >,
        /// The unique identifier number for the resource. This identifier is defined by the server.
        pub health_check_id: pulumi_gestalt_rust::Output<i32>,
        /// A so-far unhealthy instance will be marked healthy after this many
        /// consecutive successes. The default value is 2.
        pub healthy_threshold: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A nested object resource.
        /// Structure is documented below.
        pub http2_health_check: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionHealthCheckHttp2HealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        pub http_health_check: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionHealthCheckHttpHealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        pub https_health_check: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionHealthCheckHttpsHealthCheck>,
        >,
        /// Configure logging on this health check.
        /// Structure is documented below.
        pub log_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::RegionHealthCheckLogConfig,
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
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The Region in which the created health check should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A nested object resource.
        /// Structure is documented below.
        pub ssl_health_check: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionHealthCheckSslHealthCheck>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        pub tcp_health_check: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionHealthCheckTcpHealthCheck>,
        >,
        /// How long (in seconds) to wait before claiming failure.
        /// The default value is 5 seconds.  It is invalid for timeoutSec to have
        /// greater value than checkIntervalSec.
        pub timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The type of the health check. One of HTTP, HTTP2, HTTPS, TCP, or SSL.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// A so-far healthy instance will be marked unhealthy after this many
        /// consecutive failures. The default value is 2.
        pub unhealthy_threshold: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionHealthCheckArgs,
    ) -> RegionHealthCheckResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let check_interval_sec_binding = args.check_interval_sec.get_output(context);
        let description_binding = args.description.get_output(context);
        let grpc_health_check_binding = args.grpc_health_check.get_output(context);
        let healthy_threshold_binding = args.healthy_threshold.get_output(context);
        let http2_health_check_binding = args.http2_health_check.get_output(context);
        let http_health_check_binding = args.http_health_check.get_output(context);
        let https_health_check_binding = args.https_health_check.get_output(context);
        let log_config_binding = args.log_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let ssl_health_check_binding = args.ssl_health_check.get_output(context);
        let tcp_health_check_binding = args.tcp_health_check.get_output(context);
        let timeout_sec_binding = args.timeout_sec.get_output(context);
        let unhealthy_threshold_binding = args.unhealthy_threshold.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/regionHealthCheck:RegionHealthCheck".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "checkIntervalSec".into(),
                    value: check_interval_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grpcHealthCheck".into(),
                    value: grpc_health_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthyThreshold".into(),
                    value: healthy_threshold_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "http2HealthCheck".into(),
                    value: http2_health_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpHealthCheck".into(),
                    value: http_health_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsHealthCheck".into(),
                    value: https_health_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logConfig".into(),
                    value: log_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslHealthCheck".into(),
                    value: ssl_health_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tcpHealthCheck".into(),
                    value: tcp_health_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeoutSec".into(),
                    value: timeout_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "unhealthyThreshold".into(),
                    value: unhealthy_threshold_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionHealthCheckResult {
            check_interval_sec: o.get_field("checkIntervalSec"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            grpc_health_check: o.get_field("grpcHealthCheck"),
            health_check_id: o.get_field("healthCheckId"),
            healthy_threshold: o.get_field("healthyThreshold"),
            http2_health_check: o.get_field("http2HealthCheck"),
            http_health_check: o.get_field("httpHealthCheck"),
            https_health_check: o.get_field("httpsHealthCheck"),
            log_config: o.get_field("logConfig"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            ssl_health_check: o.get_field("sslHealthCheck"),
            tcp_health_check: o.get_field("tcpHealthCheck"),
            timeout_sec: o.get_field("timeoutSec"),
            type_: o.get_field("type"),
            unhealthy_threshold: o.get_field("unhealthyThreshold"),
        }
    }
}
