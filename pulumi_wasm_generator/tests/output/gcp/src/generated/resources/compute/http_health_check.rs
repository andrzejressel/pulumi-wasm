/// An HttpHealthCheck resource. This resource defines a template for how
/// individual VMs should be checked for health, via HTTP.
///
/// > **Note:** gcp.compute.HttpHealthCheck is a legacy health check.
/// The newer [gcp.compute.HealthCheck](https://www.terraform.io/docs/providers/google/r/compute_health_check.html)
/// should be preferred for all uses except
/// [Network Load Balancers](https://cloud.google.com/compute/docs/load-balancing/network/)
/// which still require the legacy version.
///
///
/// To get more information about HttpHealthCheck, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/httpHealthChecks)
/// * How-to Guides
///     * [Adding Health Checks](https://cloud.google.com/compute/docs/load-balancing/health-checks#legacy_health_checks)
///
/// ## Example Usage
///
/// ### Http Health Check Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = http_health_check::create(
///         "default",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("authentication-health-check")
///             .request_path("/health_check")
///             .timeout_sec(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// HttpHealthCheck can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/httpHealthChecks/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, HttpHealthCheck can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/httpHealthCheck:HttpHealthCheck default projects/{{project}}/global/httpHealthChecks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/httpHealthCheck:HttpHealthCheck default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/httpHealthCheck:HttpHealthCheck default {{name}}
/// ```
///
pub mod http_health_check {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HttpHealthCheckArgs {
        /// How often (in seconds) to send a health check. The default value is 5
        /// seconds.
        #[builder(into, default)]
        pub check_interval_sec: pulumi_wasm_rust::Output<Option<i32>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A so-far unhealthy instance will be marked healthy after this many
        /// consecutive successes. The default value is 2.
        #[builder(into, default)]
        pub healthy_threshold: pulumi_wasm_rust::Output<Option<i32>>,
        /// The value of the host header in the HTTP health check request. If
        /// left empty (default value), the public IP on behalf of which this
        /// health check is performed will be used.
        #[builder(into, default)]
        pub host: pulumi_wasm_rust::Output<Option<String>>,
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
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The TCP port number for the HTTP health check request.
        /// The default value is 80.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The request path of the HTTP health check request.
        /// The default value is /.
        #[builder(into, default)]
        pub request_path: pulumi_wasm_rust::Output<Option<String>>,
        /// How long (in seconds) to wait before claiming failure.
        /// The default value is 5 seconds.  It is invalid for timeoutSec to have
        /// greater value than checkIntervalSec.
        #[builder(into, default)]
        pub timeout_sec: pulumi_wasm_rust::Output<Option<i32>>,
        /// A so-far healthy instance will be marked unhealthy after this many
        /// consecutive failures. The default value is 2.
        #[builder(into, default)]
        pub unhealthy_threshold: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct HttpHealthCheckResult {
        /// How often (in seconds) to send a health check. The default value is 5
        /// seconds.
        pub check_interval_sec: pulumi_wasm_rust::Output<Option<i32>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A so-far unhealthy instance will be marked healthy after this many
        /// consecutive successes. The default value is 2.
        pub healthy_threshold: pulumi_wasm_rust::Output<Option<i32>>,
        /// The value of the host header in the HTTP health check request. If
        /// left empty (default value), the public IP on behalf of which this
        /// health check is performed will be used.
        pub host: pulumi_wasm_rust::Output<Option<String>>,
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
        /// The TCP port number for the HTTP health check request.
        /// The default value is 80.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The request path of the HTTP health check request.
        /// The default value is /.
        pub request_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// How long (in seconds) to wait before claiming failure.
        /// The default value is 5 seconds.  It is invalid for timeoutSec to have
        /// greater value than checkIntervalSec.
        pub timeout_sec: pulumi_wasm_rust::Output<Option<i32>>,
        /// A so-far healthy instance will be marked unhealthy after this many
        /// consecutive failures. The default value is 2.
        pub unhealthy_threshold: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HttpHealthCheckArgs) -> HttpHealthCheckResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let check_interval_sec_binding = args.check_interval_sec.get_inner();
        let description_binding = args.description.get_inner();
        let healthy_threshold_binding = args.healthy_threshold.get_inner();
        let host_binding = args.host.get_inner();
        let name_binding = args.name.get_inner();
        let port_binding = args.port.get_inner();
        let project_binding = args.project.get_inner();
        let request_path_binding = args.request_path.get_inner();
        let timeout_sec_binding = args.timeout_sec.get_inner();
        let unhealthy_threshold_binding = args.unhealthy_threshold.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/httpHealthCheck:HttpHealthCheck".into(),
            name: name.to_string(),
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
                    name: "healthyThreshold".into(),
                    value: &healthy_threshold_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "requestPath".into(),
                    value: &request_path_binding,
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
                    name: "healthyThreshold".into(),
                },
                register_interface::ResultField {
                    name: "host".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "requestPath".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "timeoutSec".into(),
                },
                register_interface::ResultField {
                    name: "unhealthyThreshold".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HttpHealthCheckResult {
            check_interval_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checkIntervalSec").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            healthy_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthyThreshold").unwrap(),
            ),
            host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("host").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            request_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestPath").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeoutSec").unwrap(),
            ),
            unhealthy_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("unhealthyThreshold").unwrap(),
            ),
        }
    }
}
