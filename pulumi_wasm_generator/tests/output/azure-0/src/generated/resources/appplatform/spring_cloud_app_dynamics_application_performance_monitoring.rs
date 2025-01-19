/// > **NOTE:** This resource is only applicable for Spring Cloud Service enterprise tier
///
/// Manages a Spring Cloud Application Performance Monitoring resource for App Dynamics.
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
///     let exampleSpringCloudAppDynamicsApplicationPerformanceMonitoring = spring_cloud_app_dynamics_application_performance_monitoring::create(
///         "exampleSpringCloudAppDynamicsApplicationPerformanceMonitoring",
///         SpringCloudAppDynamicsApplicationPerformanceMonitoringArgs::builder()
///             .agent_account_access_key("example-agent-account-access-key")
///             .agent_account_name("example-agent-account-name")
///             .agent_application_name("example-agent-application-name")
///             .agent_node_name("example-agent-node-name")
///             .agent_tier_name("example-agent-tier-name")
///             .agent_unique_host_id("example-agent-unique-host-id")
///             .controller_host_name("example-controller-host-name")
///             .controller_port(8080)
///             .controller_ssl_enabled(true)
///             .globally_enabled(true)
///             .name("example")
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
/// Spring Cloud Application Performance Monitoring resource for App Dynamics can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudAppDynamicsApplicationPerformanceMonitoring:SpringCloudAppDynamicsApplicationPerformanceMonitoring example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AppPlatform/spring/service1/apms/apm1
/// ```
///
pub mod spring_cloud_app_dynamics_application_performance_monitoring {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudAppDynamicsApplicationPerformanceMonitoringArgs {
        /// Specifies the account access key used to authenticate with the Controller.
        #[builder(into)]
        pub agent_account_access_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the account name of the App Dynamics account.
        #[builder(into)]
        pub agent_account_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the logical business application that this JVM node belongs to.
        #[builder(into, default)]
        pub agent_application_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the node. Where JVMs are dynamically created.
        #[builder(into, default)]
        pub agent_node_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the tier that this JVM node belongs to.
        #[builder(into, default)]
        pub agent_tier_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the unique host ID which is used to Logically partition a single physical host or virtual machine such that it appears to the Controller that the application is running on different machines.
        #[builder(into, default)]
        pub agent_unique_host_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the hostname or the IP address of the AppDynamics Controller.
        #[builder(into)]
        pub controller_host_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the HTTP(S) port of the AppDynamics Controller. This is the port used to access the AppDynamics browser-based user interface.
        #[builder(into, default)]
        pub controller_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies whether enable use SSL (HTTPS) to connect to the AppDynamics Controller.
        #[builder(into, default)]
        pub controller_ssl_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        #[builder(into, default)]
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for App Dynamics. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudAppDynamicsApplicationPerformanceMonitoringResult {
        /// Specifies the account access key used to authenticate with the Controller.
        pub agent_account_access_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the account name of the App Dynamics account.
        pub agent_account_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the logical business application that this JVM node belongs to.
        pub agent_application_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the node. Where JVMs are dynamically created.
        pub agent_node_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the tier that this JVM node belongs to.
        pub agent_tier_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the unique host ID which is used to Logically partition a single physical host or virtual machine such that it appears to the Controller that the application is running on different machines.
        pub agent_unique_host_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the hostname or the IP address of the AppDynamics Controller.
        pub controller_host_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the HTTP(S) port of the AppDynamics Controller. This is the port used to access the AppDynamics browser-based user interface.
        pub controller_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies whether enable use SSL (HTTPS) to connect to the AppDynamics Controller.
        pub controller_ssl_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether the Spring Cloud Application Performance Monitoring resource for Application Insights is enabled globally. Defaults to `false`.
        pub globally_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Application Performance Monitoring resource for App Dynamics. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new resource to be created.
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudAppDynamicsApplicationPerformanceMonitoringArgs,
    ) -> SpringCloudAppDynamicsApplicationPerformanceMonitoringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_account_access_key_binding = args.agent_account_access_key.get_inner();
        let agent_account_name_binding = args.agent_account_name.get_inner();
        let agent_application_name_binding = args.agent_application_name.get_inner();
        let agent_node_name_binding = args.agent_node_name.get_inner();
        let agent_tier_name_binding = args.agent_tier_name.get_inner();
        let agent_unique_host_id_binding = args.agent_unique_host_id.get_inner();
        let controller_host_name_binding = args.controller_host_name.get_inner();
        let controller_port_binding = args.controller_port.get_inner();
        let controller_ssl_enabled_binding = args.controller_ssl_enabled.get_inner();
        let globally_enabled_binding = args.globally_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let spring_cloud_service_id_binding = args.spring_cloud_service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudAppDynamicsApplicationPerformanceMonitoring:SpringCloudAppDynamicsApplicationPerformanceMonitoring"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentAccountAccessKey".into(),
                    value: &agent_account_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "agentAccountName".into(),
                    value: &agent_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "agentApplicationName".into(),
                    value: &agent_application_name_binding,
                },
                register_interface::ObjectField {
                    name: "agentNodeName".into(),
                    value: &agent_node_name_binding,
                },
                register_interface::ObjectField {
                    name: "agentTierName".into(),
                    value: &agent_tier_name_binding,
                },
                register_interface::ObjectField {
                    name: "agentUniqueHostId".into(),
                    value: &agent_unique_host_id_binding,
                },
                register_interface::ObjectField {
                    name: "controllerHostName".into(),
                    value: &controller_host_name_binding,
                },
                register_interface::ObjectField {
                    name: "controllerPort".into(),
                    value: &controller_port_binding,
                },
                register_interface::ObjectField {
                    name: "controllerSslEnabled".into(),
                    value: &controller_ssl_enabled_binding,
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "agentAccountAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "agentAccountName".into(),
                },
                register_interface::ResultField {
                    name: "agentApplicationName".into(),
                },
                register_interface::ResultField {
                    name: "agentNodeName".into(),
                },
                register_interface::ResultField {
                    name: "agentTierName".into(),
                },
                register_interface::ResultField {
                    name: "agentUniqueHostId".into(),
                },
                register_interface::ResultField {
                    name: "controllerHostName".into(),
                },
                register_interface::ResultField {
                    name: "controllerPort".into(),
                },
                register_interface::ResultField {
                    name: "controllerSslEnabled".into(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudAppDynamicsApplicationPerformanceMonitoringResult {
            agent_account_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentAccountAccessKey").unwrap(),
            ),
            agent_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentAccountName").unwrap(),
            ),
            agent_application_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentApplicationName").unwrap(),
            ),
            agent_node_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentNodeName").unwrap(),
            ),
            agent_tier_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentTierName").unwrap(),
            ),
            agent_unique_host_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentUniqueHostId").unwrap(),
            ),
            controller_host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controllerHostName").unwrap(),
            ),
            controller_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controllerPort").unwrap(),
            ),
            controller_ssl_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controllerSslEnabled").unwrap(),
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
        }
    }
}
