/// Manages an Azure Spring Cloud Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: tf-test-appinsights
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: S0
///       configServerGitSetting:
///         uri: https://github.com/Azure-Samples/piggymetrics
///         label: config
///         searchPaths:
///           - dir1
///           - dir2
///       trace:
///         connectionString: ${exampleInsights.connectionString}
///         sampleRate: 10
///       tags:
///         Env: staging
/// ```
///
/// ## Import
///
/// Spring Cloud services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudService:SpringCloudService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AppPlatform/spring/spring1
/// ```
///
pub mod spring_cloud_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudServiceArgs {
        /// Specifies the size for this Spring Cloud Service's default build agent pool. Possible values are `S1`, `S2`, `S3`, `S4` and `S5`. This field is applicable only for Spring Cloud Service with enterprise tier.
        #[builder(into, default)]
        pub build_agent_pool_size: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `config_server_git_setting` block as defined below. This field is applicable only for Spring Cloud Service with basic and standard tier.
        #[builder(into, default)]
        pub config_server_git_setting: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::appplatform::SpringCloudServiceConfigServerGitSetting,
            >,
        >,
        /// One or more `container_registry` block as defined below. This field is applicable only for Spring Cloud Service with enterprise tier.
        #[builder(into, default)]
        pub container_registries: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::appplatform::SpringCloudServiceContainerRegistry,
                >,
            >,
        >,
        /// A `default_build_service` block as defined below. This field is applicable only for Spring Cloud Service with enterprise tier.
        #[builder(into, default)]
        pub default_build_service: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::appplatform::SpringCloudServiceDefaultBuildService,
            >,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Should the log stream in vnet injection instance could be accessed from Internet?
        #[builder(into, default)]
        pub log_stream_public_endpoint_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The resource Id of the Managed Environment that the Spring Apps instance builds on. Can only be specified when `sku_tier` is set to `StandardGen2`.
        #[builder(into, default)]
        pub managed_environment_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `marketplace` block as defined below. Can only be specified when `sku` is set to `E0`.
        #[builder(into, default)]
        pub marketplace: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudServiceMarketplace>,
        >,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `network` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudServiceNetwork>,
        >,
        /// Specifies The name of the resource group in which to create the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether enable the default Service Registry. This field is applicable only for Spring Cloud Service with enterprise tier.
        #[builder(into, default)]
        pub service_registry_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the SKU Name for this Spring Cloud Service. Possible values are `B0`, `S0` and `E0`. Defaults to `S0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the SKU Tier for this Spring Cloud Service. Possible values are `Basic`, `Enterprise`, `Standard` and `StandardGen2`. The attribute is automatically computed from API response except when `managed_environment_id` is defined. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku_tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `trace` block as defined below.
        #[builder(into, default)]
        pub trace: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudServiceTrace>,
        >,
        /// Whether zone redundancy is enabled for this Spring Cloud Service. Defaults to `false`.
        #[builder(into, default)]
        pub zone_redundant: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudServiceResult {
        /// Specifies the size for this Spring Cloud Service's default build agent pool. Possible values are `S1`, `S2`, `S3`, `S4` and `S5`. This field is applicable only for Spring Cloud Service with enterprise tier.
        pub build_agent_pool_size: pulumi_wasm_rust::Output<Option<String>>,
        /// A `config_server_git_setting` block as defined below. This field is applicable only for Spring Cloud Service with basic and standard tier.
        pub config_server_git_setting: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudServiceConfigServerGitSetting,
            >,
        >,
        /// One or more `container_registry` block as defined below. This field is applicable only for Spring Cloud Service with enterprise tier.
        pub container_registries: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appplatform::SpringCloudServiceContainerRegistry,
                >,
            >,
        >,
        /// A `default_build_service` block as defined below. This field is applicable only for Spring Cloud Service with enterprise tier.
        pub default_build_service: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudServiceDefaultBuildService,
            >,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Should the log stream in vnet injection instance could be accessed from Internet?
        pub log_stream_public_endpoint_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The resource Id of the Managed Environment that the Spring Apps instance builds on. Can only be specified when `sku_tier` is set to `StandardGen2`.
        pub managed_environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `marketplace` block as defined below. Can only be specified when `sku` is set to `E0`.
        pub marketplace: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudServiceMarketplace,
        >,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network` block as defined below. Changing this forces a new resource to be created.
        pub network: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudServiceNetwork>,
        >,
        /// A list of the outbound Public IP Addresses used by this Spring Cloud Service.
        pub outbound_public_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of `required_network_traffic_rules` blocks as defined below.
        pub required_network_traffic_rules: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::appplatform::SpringCloudServiceRequiredNetworkTrafficRule,
            >,
        >,
        /// Specifies The name of the resource group in which to create the Spring Cloud Service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Whether enable the default Service Registry. This field is applicable only for Spring Cloud Service with enterprise tier.
        pub service_registry_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Spring Cloud Service Registry.
        pub service_registry_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the SKU Name for this Spring Cloud Service. Possible values are `B0`, `S0` and `E0`. Defaults to `S0`. Changing this forces a new resource to be created.
        pub sku_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the SKU Tier for this Spring Cloud Service. Possible values are `Basic`, `Enterprise`, `Standard` and `StandardGen2`. The attribute is automatically computed from API response except when `managed_environment_id` is defined. Changing this forces a new resource to be created.
        pub sku_tier: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `trace` block as defined below.
        pub trace: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudServiceTrace>,
        >,
        /// Whether zone redundancy is enabled for this Spring Cloud Service. Defaults to `false`.
        pub zone_redundant: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SpringCloudServiceArgs,
    ) -> SpringCloudServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let build_agent_pool_size_binding = args
            .build_agent_pool_size
            .get_output(context)
            .get_inner();
        let config_server_git_setting_binding = args
            .config_server_git_setting
            .get_output(context)
            .get_inner();
        let container_registries_binding = args
            .container_registries
            .get_output(context)
            .get_inner();
        let default_build_service_binding = args
            .default_build_service
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let log_stream_public_endpoint_enabled_binding = args
            .log_stream_public_endpoint_enabled
            .get_output(context)
            .get_inner();
        let managed_environment_id_binding = args
            .managed_environment_id
            .get_output(context)
            .get_inner();
        let marketplace_binding = args.marketplace.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let service_registry_enabled_binding = args
            .service_registry_enabled
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let sku_tier_binding = args.sku_tier.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let trace_binding = args.trace.get_output(context).get_inner();
        let zone_redundant_binding = args.zone_redundant.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudService:SpringCloudService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "buildAgentPoolSize".into(),
                    value: &build_agent_pool_size_binding,
                },
                register_interface::ObjectField {
                    name: "configServerGitSetting".into(),
                    value: &config_server_git_setting_binding,
                },
                register_interface::ObjectField {
                    name: "containerRegistries".into(),
                    value: &container_registries_binding,
                },
                register_interface::ObjectField {
                    name: "defaultBuildService".into(),
                    value: &default_build_service_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logStreamPublicEndpointEnabled".into(),
                    value: &log_stream_public_endpoint_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "managedEnvironmentId".into(),
                    value: &managed_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "marketplace".into(),
                    value: &marketplace_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceRegistryEnabled".into(),
                    value: &service_registry_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuTier".into(),
                    value: &sku_tier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trace".into(),
                    value: &trace_binding,
                },
                register_interface::ObjectField {
                    name: "zoneRedundant".into(),
                    value: &zone_redundant_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudServiceResult {
            build_agent_pool_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("buildAgentPoolSize"),
            ),
            config_server_git_setting: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configServerGitSetting"),
            ),
            container_registries: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerRegistries"),
            ),
            default_build_service: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultBuildService"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            log_stream_public_endpoint_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logStreamPublicEndpointEnabled"),
            ),
            managed_environment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managedEnvironmentId"),
            ),
            marketplace: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("marketplace"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            outbound_public_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outboundPublicIpAddresses"),
            ),
            required_network_traffic_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requiredNetworkTrafficRules"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            service_registry_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceRegistryEnabled"),
            ),
            service_registry_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceRegistryId"),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            sku_tier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuTier"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            trace: pulumi_wasm_rust::__private::into_domain(o.extract_field("trace")),
            zone_redundant: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("zoneRedundant"),
            ),
        }
    }
}
