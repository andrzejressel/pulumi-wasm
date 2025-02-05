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
pub mod spring_cloud_new_relic_application_performance_monitoring {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudNewRelicApplicationPerformanceMonitoringArgs {
        /// Specifies whether enable the agent. Defaults to `true`.
        #[builder(into, default)]
        pub agent_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the application name used to report data to New Relic.
        #[builder(into)]
        pub app_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the port number to differentiate JVMs for the same app on the same machine.
        #[builder(into, default)]
        pub app_server_port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies whether enable plain text logging of all data sent to New Relic to the agent logfile. Defaults to `false`.
        #[builder(into, default)]
        pub audit_mode_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether enable the reporting of data separately for each web app. Defaults to `false`.
        #[builder(into, default)]
        pub auto_app_naming_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether enable the component-based transaction naming. Defaults to `true`.
        #[builder(into, default)]
        pub auto_transaction_naming_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether enable all instrumentation using an `@Trace` annotation. Disabling this causes `@Trace` annotations to be ignored. Defaults to `true`.
        #[builder(into, default)]
        pub custom_tracing_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies a mapping of labels to be added to the New Relic application.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the license key associated with the New Relic account. This key binds your agent's data to your account in New Relic service.
        #[builder(into)]
        pub license_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for New Relic. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudNewRelicApplicationPerformanceMonitoringResult {
        /// Specifies whether enable the agent. Defaults to `true`.
        pub agent_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the application name used to report data to New Relic.
        pub app_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the port number to differentiate JVMs for the same app on the same machine.
        pub app_server_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies whether enable plain text logging of all data sent to New Relic to the agent logfile. Defaults to `false`.
        pub audit_mode_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether enable the reporting of data separately for each web app. Defaults to `false`.
        pub auto_app_naming_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether enable the component-based transaction naming. Defaults to `true`.
        pub auto_transaction_naming_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether enable all instrumentation using an `@Trace` annotation. Disabling this causes `@Trace` annotations to be ignored. Defaults to `true`.
        pub custom_tracing_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies a mapping of labels to be added to the New Relic application.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the license key associated with the New Relic account. This key binds your agent's data to your account in New Relic service.
        pub license_key: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for New Relic. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SpringCloudNewRelicApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudNewRelicApplicationPerformanceMonitoringResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_enabled_binding = args.agent_enabled.get_output(context).get_inner();
        let app_name_binding = args.app_name.get_output(context).get_inner();
        let app_server_port_binding = args
            .app_server_port
            .get_output(context)
            .get_inner();
        let audit_mode_enabled_binding = args
            .audit_mode_enabled
            .get_output(context)
            .get_inner();
        let auto_app_naming_enabled_binding = args
            .auto_app_naming_enabled
            .get_output(context)
            .get_inner();
        let auto_transaction_naming_enabled_binding = args
            .auto_transaction_naming_enabled
            .get_output(context)
            .get_inner();
        let custom_tracing_enabled_binding = args
            .custom_tracing_enabled
            .get_output(context)
            .get_inner();
        let globally_enabled_binding = args
            .globally_enabled
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let license_key_binding = args.license_key.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudNewRelicApplicationPerformanceMonitoring:SpringCloudNewRelicApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentEnabled".into(),
                    value: &agent_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "appName".into(),
                    value: &app_name_binding,
                },
                register_interface::ObjectField {
                    name: "appServerPort".into(),
                    value: &app_server_port_binding,
                },
                register_interface::ObjectField {
                    name: "auditModeEnabled".into(),
                    value: &audit_mode_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "autoAppNamingEnabled".into(),
                    value: &auto_app_naming_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "autoTransactionNamingEnabled".into(),
                    value: &auto_transaction_naming_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "customTracingEnabled".into(),
                    value: &custom_tracing_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "globallyEnabled".into(),
                    value: &globally_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "licenseKey".into(),
                    value: &license_key_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudNewRelicApplicationPerformanceMonitoringResult {
            agent_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentEnabled"),
            ),
            app_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appName"),
            ),
            app_server_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appServerPort"),
            ),
            audit_mode_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("auditModeEnabled"),
            ),
            auto_app_naming_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoAppNamingEnabled"),
            ),
            auto_transaction_naming_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoTransactionNamingEnabled"),
            ),
            custom_tracing_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customTracingEnabled"),
            ),
            globally_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("globallyEnabled"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            license_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("licenseKey"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            spring_cloud_service_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("springCloudServiceId"),
            ),
        }
    }
}
