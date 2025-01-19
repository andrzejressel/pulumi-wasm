/// > **NOTE:** This resource is only applicable for Spring Cloud Service enterprise tier
///
/// Manages a Spring Cloud Application Performance Monitoring resource for Elastic.
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
///     let exampleSpringCloudElasticApplicationPerformanceMonitoring = spring_cloud_elastic_application_performance_monitoring::create(
///         "exampleSpringCloudElasticApplicationPerformanceMonitoring",
///         SpringCloudElasticApplicationPerformanceMonitoringArgs::builder()
///             .application_packages(vec!["org.example", "org.another.example",])
///             .globally_enabled(true)
///             .name("example")
///             .server_url("http://127.0.0.1:8200")
///             .service_name("example-service-name")
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
/// Spring Cloud Application Performance Monitoring resource for Elastic can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudElasticApplicationPerformanceMonitoring:SpringCloudElasticApplicationPerformanceMonitoring example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AppPlatform/spring/service1/apms/apm1
/// ```
///
pub mod spring_cloud_elastic_application_performance_monitoring {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudElasticApplicationPerformanceMonitoringArgs {
        /// Specifies a list of the packages which should be used to determine whether a stack trace frame is an in-app frame or a library frame. This is a comma separated list of package names.
        #[builder(into)]
        pub application_packages: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Elastic. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the server URL. The URL must be fully qualified, including protocol (http or https) and port.
        #[builder(into)]
        pub server_url: pulumi_wasm_rust::Output<String>,
        /// Specifies the service name which is used to keep all the errors and transactions of your service together and is the primary filter in the Elastic APM user interface.
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudElasticApplicationPerformanceMonitoringResult {
        /// Specifies a list of the packages which should be used to determine whether a stack trace frame is an in-app frame or a library frame. This is a comma separated list of package names.
        pub application_packages: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Elastic. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the server URL. The URL must be fully qualified, including protocol (http or https) and port.
        pub server_url: pulumi_wasm_rust::Output<String>,
        /// Specifies the service name which is used to keep all the errors and transactions of your service together and is the primary filter in the Elastic APM user interface.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudElasticApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudElasticApplicationPerformanceMonitoringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_packages_binding = args.application_packages.get_inner();
        let globally_enabled_binding = args.globally_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let server_url_binding = args.server_url.get_inner();
        let service_name_binding = args.service_name.get_inner();
        let spring_cloud_service_id_binding = args.spring_cloud_service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudElasticApplicationPerformanceMonitoring:SpringCloudElasticApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationPackages".into(),
                    value: &application_packages_binding,
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
                    name: "serverUrl".into(),
                    value: &server_url_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationPackages".into(),
                },
                register_interface::ResultField {
                    name: "globallyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serverUrl".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
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
        SpringCloudElasticApplicationPerformanceMonitoringResult {
            application_packages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationPackages").unwrap(),
            ),
            globally_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globallyEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            server_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverUrl").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            spring_cloud_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudServiceId").unwrap(),
            ),
        }
    }
}
