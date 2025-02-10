/// > **NOTE:** This resource is only applicable for Spring Cloud Service enterprise tier
///
/// Manages a Spring Cloud Application Performance Monitoring resource for Dynatrace.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_dynatrace_application_performance_monitoring {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudDynatraceApplicationPerformanceMonitoringArgs {
        /// Specifies the API token of the Dynatrace environment.
        #[builder(into, default)]
        pub api_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the API Url of the Dynatrace environment.
        #[builder(into, default)]
        pub api_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the endpoint to connect to the Dynatrace environment.
        #[builder(into)]
        pub connection_point: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Dynatrace environment ID.
        #[builder(into, default)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Dynatrace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Dynatrace tenant.
        #[builder(into)]
        pub tenant: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the internal token that is used for authentication when OneAgent connects to the Dynatrace cluster to send data.
        #[builder(into)]
        pub tenant_token: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudDynatraceApplicationPerformanceMonitoringResult {
        /// Specifies the API token of the Dynatrace environment.
        pub api_token: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the API Url of the Dynatrace environment.
        pub api_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the endpoint to connect to the Dynatrace environment.
        pub connection_point: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Dynatrace environment ID.
        pub environment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for Dynatrace. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Dynatrace tenant.
        pub tenant: pulumi_gestalt_rust::Output<String>,
        /// Specifies the internal token that is used for authentication when OneAgent connects to the Dynatrace cluster to send data.
        pub tenant_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudDynatraceApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudDynatraceApplicationPerformanceMonitoringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_token_binding = args.api_token.get_output(context);
        let api_url_binding = args.api_url.get_output(context);
        let connection_point_binding = args.connection_point.get_output(context);
        let environment_id_binding = args.environment_id.get_output(context);
        let globally_enabled_binding = args.globally_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context);
        let tenant_binding = args.tenant.get_output(context);
        let tenant_token_binding = args.tenant_token.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudDynatraceApplicationPerformanceMonitoring:SpringCloudDynatraceApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiToken".into(),
                    value: api_token_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiUrl".into(),
                    value: api_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionPoint".into(),
                    value: connection_point_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentId".into(),
                    value: environment_id_binding.get_id(),
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
                    name: "springCloudServiceId".into(),
                    value: spring_cloud_service_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenant".into(),
                    value: tenant_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantToken".into(),
                    value: tenant_token_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudDynatraceApplicationPerformanceMonitoringResult {
            api_token: o.get_field("apiToken"),
            api_url: o.get_field("apiUrl"),
            connection_point: o.get_field("connectionPoint"),
            environment_id: o.get_field("environmentId"),
            globally_enabled: o.get_field("globallyEnabled"),
            name: o.get_field("name"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
            tenant: o.get_field("tenant"),
            tenant_token: o.get_field("tenantToken"),
        }
    }
}
