pub mod get_health_check {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHealthCheckArgs {
        /// Name of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetHealthCheckResult {
        pub check_interval_sec: pulumi_wasm_rust::Output<i32>,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub grpc_health_checks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckGrpcHealthCheck>,
        >,
        pub healthy_threshold: pulumi_wasm_rust::Output<i32>,
        pub http2_health_checks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckHttp2HealthCheck>,
        >,
        pub http_health_checks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckHttpHealthCheck>,
        >,
        pub https_health_checks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckHttpsHealthCheck>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub log_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckLogConfig>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub source_regions: pulumi_wasm_rust::Output<Vec<String>>,
        pub ssl_health_checks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckSslHealthCheck>,
        >,
        pub tcp_health_checks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckTcpHealthCheck>,
        >,
        pub timeout_sec: pulumi_wasm_rust::Output<i32>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub unhealthy_threshold: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetHealthCheckArgs) -> GetHealthCheckResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getHealthCheck:getHealthCheck".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
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
                    name: "grpcHealthChecks".into(),
                },
                register_interface::ResultField {
                    name: "healthyThreshold".into(),
                },
                register_interface::ResultField {
                    name: "http2HealthChecks".into(),
                },
                register_interface::ResultField {
                    name: "httpHealthChecks".into(),
                },
                register_interface::ResultField {
                    name: "httpsHealthChecks".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "logConfigs".into(),
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
                    name: "sslHealthChecks".into(),
                },
                register_interface::ResultField {
                    name: "tcpHealthChecks".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetHealthCheckResult {
            check_interval_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checkIntervalSec").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            grpc_health_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grpcHealthChecks").unwrap(),
            ),
            healthy_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthyThreshold").unwrap(),
            ),
            http2_health_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("http2HealthChecks").unwrap(),
            ),
            http_health_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpHealthChecks").unwrap(),
            ),
            https_health_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsHealthChecks").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            log_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logConfigs").unwrap(),
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
            ssl_health_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslHealthChecks").unwrap(),
            ),
            tcp_health_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tcpHealthChecks").unwrap(),
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
