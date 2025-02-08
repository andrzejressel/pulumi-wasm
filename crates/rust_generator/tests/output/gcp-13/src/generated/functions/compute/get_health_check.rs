#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_health_check {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHealthCheckArgs {
        /// Name of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetHealthCheckResult {
        pub check_interval_sec: pulumi_gestalt_rust::Output<i32>,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub grpc_health_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckGrpcHealthCheck>,
        >,
        pub healthy_threshold: pulumi_gestalt_rust::Output<i32>,
        pub http2_health_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckHttp2HealthCheck>,
        >,
        pub http_health_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckHttpHealthCheck>,
        >,
        pub https_health_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckHttpsHealthCheck>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub log_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckLogConfig>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub source_regions: pulumi_gestalt_rust::Output<Vec<String>>,
        pub ssl_health_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckSslHealthCheck>,
        >,
        pub tcp_health_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetHealthCheckTcpHealthCheck>,
        >,
        pub timeout_sec: pulumi_gestalt_rust::Output<i32>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub unhealthy_threshold: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetHealthCheckArgs,
    ) -> GetHealthCheckResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetHealthCheckResult {
            check_interval_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("checkIntervalSec"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            grpc_health_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grpcHealthChecks"),
            ),
            healthy_threshold: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("healthyThreshold"),
            ),
            http2_health_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("http2HealthChecks"),
            ),
            http_health_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpHealthChecks"),
            ),
            https_health_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsHealthChecks"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            log_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logConfigs"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            source_regions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceRegions"),
            ),
            ssl_health_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslHealthChecks"),
            ),
            tcp_health_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tcpHealthChecks"),
            ),
            timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeoutSec"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            unhealthy_threshold: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("unhealthyThreshold"),
            ),
        }
    }
}
