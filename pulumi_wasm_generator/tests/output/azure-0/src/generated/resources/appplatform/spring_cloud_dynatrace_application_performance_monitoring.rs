/// > **NOTE:** This resource is only applicable for Spring Cloud Service enterprise tier
///
/// Manages a Spring Cloud Application Performance Monitoring resource for Dynatrace.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleSpringCloudDynatraceApplicationPerformanceMonitoring = spring_cloud_dynatrace_application_performance_monitoring::create(
///         "exampleSpringCloudDynatraceApplicationPerformanceMonitoring",
///         SpringCloudDynatraceApplicationPerformanceMonitoringArgs::builder()
///             .api_token(
///                 "dt0s01.AAAAAAAAAAAAAAAAAAAAAAAA.BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
///             )
///             .api_url("https://example-api-url.com")
///             .connection_point("https://example.live.dynatrace.com:443")
///             .environment_id("example-environment-id")
///             .globally_enabled(true)
///             .name("example")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .tenant("example-tenant")
///             .tenant_token(
///                 "dt0s01.AAAAAAAAAAAAAAAAAAAAAAAA.BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
///             )
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku_name("E0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Application Performance Monitoring resource for Dynatrace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudDynatraceApplicationPerformanceMonitoring:SpringCloudDynatraceApplicationPerformanceMonitoring example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AppPlatform/spring/service1/apms/apm1
/// ```
///
pub mod spring_cloud_dynatrace_application_performance_monitoring {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudDynatraceApplicationPerformanceMonitoringArgs {
        /// Specifies the API token of the Dynatrace environment.
        #[builder(into, default)]
        pub api_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the API Url of the Dynatrace environment.
        #[builder(into, default)]
        pub api_url: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the endpoint to connect to the Dynatrace environment.
        #[builder(into)]
        pub connection_point: pulumi_wasm_rust::Output<String>,
        /// Specifies the Dynatrace environment ID.
        #[builder(into, default)]
        pub environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Dynatrace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Dynatrace tenant.
        #[builder(into)]
        pub tenant: pulumi_wasm_rust::Output<String>,
        /// Specifies the internal token that is used for authentication when OneAgent connects to the Dynatrace cluster to send data.
        #[builder(into)]
        pub tenant_token: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudDynatraceApplicationPerformanceMonitoringResult {
        /// Specifies the API token of the Dynatrace environment.
        pub api_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the API Url of the Dynatrace environment.
        pub api_url: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the endpoint to connect to the Dynatrace environment.
        pub connection_point: pulumi_wasm_rust::Output<String>,
        /// Specifies the Dynatrace environment ID.
        pub environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Dynatrace. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Dynatrace tenant.
        pub tenant: pulumi_wasm_rust::Output<String>,
        /// Specifies the internal token that is used for authentication when OneAgent connects to the Dynatrace cluster to send data.
        pub tenant_token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudDynatraceApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudDynatraceApplicationPerformanceMonitoringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_token_binding = args.api_token.get_inner();
        let api_url_binding = args.api_url.get_inner();
        let connection_point_binding = args.connection_point.get_inner();
        let environment_id_binding = args.environment_id.get_inner();
        let globally_enabled_binding = args.globally_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let spring_cloud_service_id_binding = args.spring_cloud_service_id.get_inner();
        let tenant_binding = args.tenant.get_inner();
        let tenant_token_binding = args.tenant_token.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudDynatraceApplicationPerformanceMonitoring:SpringCloudDynatraceApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiToken".into(),
                    value: &api_token_binding,
                },
                register_interface::ObjectField {
                    name: "apiUrl".into(),
                    value: &api_url_binding,
                },
                register_interface::ObjectField {
                    name: "connectionPoint".into(),
                    value: &connection_point_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "globallyEnabled".into(),
                    value: &globally_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "tenant".into(),
                    value: &tenant_binding,
                },
                register_interface::ObjectField {
                    name: "tenantToken".into(),
                    value: &tenant_token_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiToken".into(),
                },
                register_interface::ResultField {
                    name: "apiUrl".into(),
                },
                register_interface::ResultField {
                    name: "connectionPoint".into(),
                },
                register_interface::ResultField {
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "globallyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "springCloudServiceId".into(),
                },
                register_interface::ResultField {
                    name: "tenant".into(),
                },
                register_interface::ResultField {
                    name: "tenantToken".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudDynatraceApplicationPerformanceMonitoringResult {
            api_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiToken").unwrap(),
            ),
            api_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiUrl").unwrap(),
            ),
            connection_point: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionPoint").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            globally_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globallyEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            spring_cloud_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudServiceId").unwrap(),
            ),
            tenant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenant").unwrap(),
            ),
            tenant_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantToken").unwrap(),
            ),
        }
    }
}
