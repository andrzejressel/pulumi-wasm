/// > **NOTE:** This resource is only applicable for Spring Cloud Service enterprise tier
///
/// Manages a Spring Cloud Application Performance Monitoring resource for New Relic.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: E0
///   exampleSpringCloudNewRelicApplicationPerformanceMonitoring:
///     type: azure:appplatform:SpringCloudNewRelicApplicationPerformanceMonitoring
///     name: example
///     properties:
///       name: example
///       springCloudServiceId: ${exampleSpringCloudService.id}
///       appName: example-app-name
///       licenseKey: example-license-key
///       appServerPort: 8080
///       labels:
///         tagName1: tagValue1
///         tagName2: tagValue2
///       globallyEnabled: true
/// ```
///
/// ## Import
///
/// Spring Cloud Application Performance Monitoring resource for New Relic can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudNewRelicApplicationPerformanceMonitoring:SpringCloudNewRelicApplicationPerformanceMonitoring example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AppPlatform/spring/service1/apms/apm1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_new_relic_application_performance_monitoring {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudNewRelicApplicationPerformanceMonitoringArgs {
        /// Specifies whether enable the agent. Defaults to `true`.
        #[builder(into, default)]
        pub agent_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the application name used to report data to New Relic.
        #[builder(into)]
        pub app_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the port number to differentiate JVMs for the same app on the same machine.
        #[builder(into, default)]
        pub app_server_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies whether enable plain text logging of all data sent to New Relic to the agent logfile. Defaults to `false`.
        #[builder(into, default)]
        pub audit_mode_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether enable the reporting of data separately for each web app. Defaults to `false`.
        #[builder(into, default)]
        pub auto_app_naming_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether enable the component-based transaction naming. Defaults to `true`.
        #[builder(into, default)]
        pub auto_transaction_naming_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether enable all instrumentation using an `@Trace` annotation. Disabling this causes `@Trace` annotations to be ignored. Defaults to `true`.
        #[builder(into, default)]
        pub custom_tracing_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies a mapping of labels to be added to the New Relic application.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the license key associated with the New Relic account. This key binds your agent's data to your account in New Relic service.
        #[builder(into)]
        pub license_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for New Relic. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudNewRelicApplicationPerformanceMonitoringResult {
        /// Specifies whether enable the agent. Defaults to `true`.
        pub agent_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the application name used to report data to New Relic.
        pub app_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the port number to differentiate JVMs for the same app on the same machine.
        pub app_server_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies whether enable plain text logging of all data sent to New Relic to the agent logfile. Defaults to `false`.
        pub audit_mode_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether enable the reporting of data separately for each web app. Defaults to `false`.
        pub auto_app_naming_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether enable the component-based transaction naming. Defaults to `true`.
        pub auto_transaction_naming_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether enable all instrumentation using an `@Trace` annotation. Disabling this causes `@Trace` annotations to be ignored. Defaults to `true`.
        pub custom_tracing_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies a mapping of labels to be added to the New Relic application.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the license key associated with the New Relic account. This key binds your agent's data to your account in New Relic service.
        pub license_key: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for New Relic. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
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
        args: SpringCloudNewRelicApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudNewRelicApplicationPerformanceMonitoringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_enabled_binding = args.agent_enabled.get_output(context);
        let app_name_binding = args.app_name.get_output(context);
        let app_server_port_binding = args.app_server_port.get_output(context);
        let audit_mode_enabled_binding = args.audit_mode_enabled.get_output(context);
        let auto_app_naming_enabled_binding = args
            .auto_app_naming_enabled
            .get_output(context);
        let auto_transaction_naming_enabled_binding = args
            .auto_transaction_naming_enabled
            .get_output(context);
        let custom_tracing_enabled_binding = args
            .custom_tracing_enabled
            .get_output(context);
        let globally_enabled_binding = args.globally_enabled.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let license_key_binding = args.license_key.get_output(context);
        let name_binding = args.name.get_output(context);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudNewRelicApplicationPerformanceMonitoring:SpringCloudNewRelicApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentEnabled".into(),
                    value: &agent_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appName".into(),
                    value: &app_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServerPort".into(),
                    value: &app_server_port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditModeEnabled".into(),
                    value: &audit_mode_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoAppNamingEnabled".into(),
                    value: &auto_app_naming_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoTransactionNamingEnabled".into(),
                    value: &auto_transaction_naming_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customTracingEnabled".into(),
                    value: &custom_tracing_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globallyEnabled".into(),
                    value: &globally_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseKey".into(),
                    value: &license_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudNewRelicApplicationPerformanceMonitoringResult {
            agent_enabled: o.get_field("agentEnabled"),
            app_name: o.get_field("appName"),
            app_server_port: o.get_field("appServerPort"),
            audit_mode_enabled: o.get_field("auditModeEnabled"),
            auto_app_naming_enabled: o.get_field("autoAppNamingEnabled"),
            auto_transaction_naming_enabled: o.get_field("autoTransactionNamingEnabled"),
            custom_tracing_enabled: o.get_field("customTracingEnabled"),
            globally_enabled: o.get_field("globallyEnabled"),
            labels: o.get_field("labels"),
            license_key: o.get_field("licenseKey"),
            name: o.get_field("name"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
        }
    }
}
