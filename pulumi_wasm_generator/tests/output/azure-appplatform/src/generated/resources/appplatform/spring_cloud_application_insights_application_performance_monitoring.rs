/// > **NOTE:** This resource is only applicable for Spring Cloud Service enterprise tier
///
/// Manages a Spring Cloud Application Performance Monitoring resource for Application Insights.
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
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudApplicationInsightsApplicationPerformanceMonitoring = spring_cloud_application_insights_application_performance_monitoring::create(
///         "exampleSpringCloudApplicationInsightsApplicationPerformanceMonitoring",
///         SpringCloudApplicationInsightsApplicationPerformanceMonitoringArgs::builder()
///             .connection_string("${exampleInsights.instrumentationKey}")
///             .globally_enabled(true)
///             .name("example")
///             .role_instance("test-instance")
///             .role_name("test-role")
///             .sampling_percentage(50)
///             .sampling_requests_per_second(10)
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
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
/// Spring Cloud Application Performance Monitoring resource for Application Insights can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudApplicationInsightsApplicationPerformanceMonitoring:SpringCloudApplicationInsightsApplicationPerformanceMonitoring example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AppPlatform/spring/service1/apms/apm1
/// ```
///
pub mod spring_cloud_application_insights_application_performance_monitoring {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudApplicationInsightsApplicationPerformanceMonitoringArgs {
        /// The instrumentation key used to push data to Application Insights.
        #[builder(into, default)]
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Application Insights. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the cloud role instance.
        #[builder(into, default)]
        pub role_instance: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the cloud role name used to label the component on the application map.
        #[builder(into, default)]
        pub role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the percentage for fixed-percentage sampling.
        #[builder(into, default)]
        pub sampling_percentage: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the number of requests per second for the rate-limited sampling.
        #[builder(into, default)]
        pub sampling_requests_per_second: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudApplicationInsightsApplicationPerformanceMonitoringResult {
        /// The instrumentation key used to push data to Application Insights.
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Application Insights. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the cloud role instance.
        pub role_instance: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the cloud role name used to label the component on the application map.
        pub role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the percentage for fixed-percentage sampling.
        pub sampling_percentage: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the number of requests per second for the rate-limited sampling.
        pub sampling_requests_per_second: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudApplicationInsightsApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudApplicationInsightsApplicationPerformanceMonitoringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_string_binding = args.connection_string.get_inner();
        let globally_enabled_binding = args.globally_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let role_instance_binding = args.role_instance.get_inner();
        let role_name_binding = args.role_name.get_inner();
        let sampling_percentage_binding = args.sampling_percentage.get_inner();
        let sampling_requests_per_second_binding = args
            .sampling_requests_per_second
            .get_inner();
        let spring_cloud_service_id_binding = args.spring_cloud_service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudApplicationInsightsApplicationPerformanceMonitoring:SpringCloudApplicationInsightsApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
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
                    name: "roleInstance".into(),
                    value: &role_instance_binding,
                },
                register_interface::ObjectField {
                    name: "roleName".into(),
                    value: &role_name_binding,
                },
                register_interface::ObjectField {
                    name: "samplingPercentage".into(),
                    value: &sampling_percentage_binding,
                },
                register_interface::ObjectField {
                    name: "samplingRequestsPerSecond".into(),
                    value: &sampling_requests_per_second_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "globallyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleInstance".into(),
                },
                register_interface::ResultField {
                    name: "roleName".into(),
                },
                register_interface::ResultField {
                    name: "samplingPercentage".into(),
                },
                register_interface::ResultField {
                    name: "samplingRequestsPerSecond".into(),
                },
                register_interface::ResultField {
                    name: "springCloudServiceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudApplicationInsightsApplicationPerformanceMonitoringResult {
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            globally_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globallyEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleInstance").unwrap(),
            ),
            role_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleName").unwrap(),
            ),
            sampling_percentage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samplingPercentage").unwrap(),
            ),
            sampling_requests_per_second: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samplingRequestsPerSecond").unwrap(),
            ),
            spring_cloud_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudServiceId").unwrap(),
            ),
        }
    }
}
