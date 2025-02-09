/// > **NOTE:** This resource is only applicable for Spring Cloud Service enterprise tier
///
/// Manages a Spring Cloud Application Performance Monitoring resource for Application Insights.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_application_insights_application_performance_monitoring {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudApplicationInsightsApplicationPerformanceMonitoringArgs {
        /// The instrumentation key used to push data to Application Insights.
        #[builder(into, default)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Application Insights. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the cloud role instance.
        #[builder(into, default)]
        pub role_instance: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the cloud role name used to label the component on the application map.
        #[builder(into, default)]
        pub role_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the percentage for fixed-percentage sampling.
        #[builder(into, default)]
        pub sampling_percentage: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the number of requests per second for the rate-limited sampling.
        #[builder(into, default)]
        pub sampling_requests_per_second: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudApplicationInsightsApplicationPerformanceMonitoringResult {
        /// The instrumentation key used to push data to Application Insights.
        pub connection_string: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Application Insights. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the cloud role instance.
        pub role_instance: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the cloud role name used to label the component on the application map.
        pub role_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the percentage for fixed-percentage sampling.
        pub sampling_percentage: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the number of requests per second for the rate-limited sampling.
        pub sampling_requests_per_second: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudApplicationInsightsApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudApplicationInsightsApplicationPerformanceMonitoringResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_string_binding = args.connection_string.get_output(context);
        let globally_enabled_binding = args.globally_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_instance_binding = args.role_instance.get_output(context);
        let role_name_binding = args.role_name.get_output(context);
        let sampling_percentage_binding = args.sampling_percentage.get_output(context);
        let sampling_requests_per_second_binding = args
            .sampling_requests_per_second
            .get_output(context);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudApplicationInsightsApplicationPerformanceMonitoring:SpringCloudApplicationInsightsApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionString".into(),
                    value: connection_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globallyEnabled".into(),
                    value: globally_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleInstance".into(),
                    value: role_instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleName".into(),
                    value: role_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samplingPercentage".into(),
                    value: sampling_percentage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samplingRequestsPerSecond".into(),
                    value: sampling_requests_per_second_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudServiceId".into(),
                    value: spring_cloud_service_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudApplicationInsightsApplicationPerformanceMonitoringResult {
            connection_string: o.get_field("connectionString"),
            globally_enabled: o.get_field("globallyEnabled"),
            name: o.get_field("name"),
            role_instance: o.get_field("roleInstance"),
            role_name: o.get_field("roleName"),
            sampling_percentage: o.get_field("samplingPercentage"),
            sampling_requests_per_second: o.get_field("samplingRequestsPerSecond"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
        }
    }
}
