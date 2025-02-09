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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudServiceArgs {
        /// Specifies the size for this Spring Cloud Service's default build agent pool. Possible values are `S1`, `S2`, `S3`, `S4` and `S5`. This field is applicable only for Spring Cloud Service with enterprise tier.
        #[builder(into, default)]
        pub build_agent_pool_size: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `config_server_git_setting` block as defined below. This field is applicable only for Spring Cloud Service with basic and standard tier.
        #[builder(into, default)]
        pub config_server_git_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appplatform::SpringCloudServiceConfigServerGitSetting,
            >,
        >,
        /// One or more `container_registry` block as defined below. This field is applicable only for Spring Cloud Service with enterprise tier.
        #[builder(into, default)]
        pub container_registries: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::appplatform::SpringCloudServiceContainerRegistry,
                >,
            >,
        >,
        /// A `default_build_service` block as defined below. This field is applicable only for Spring Cloud Service with enterprise tier.
        #[builder(into, default)]
        pub default_build_service: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appplatform::SpringCloudServiceDefaultBuildService,
            >,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the log stream in vnet injection instance could be accessed from Internet?
        #[builder(into, default)]
        pub log_stream_public_endpoint_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The resource Id of the Managed Environment that the Spring Apps instance builds on. Can only be specified when `sku_tier` is set to `StandardGen2`.
        #[builder(into, default)]
        pub managed_environment_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `marketplace` block as defined below. Can only be specified when `sku` is set to `E0`.
        #[builder(into, default)]
        pub marketplace: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudServiceMarketplace>,
        >,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudServiceNetwork>,
        >,
        /// Specifies The name of the resource group in which to create the Spring Cloud Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether enable the default Service Registry. This field is applicable only for Spring Cloud Service with enterprise tier.
        #[builder(into, default)]
        pub service_registry_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the SKU Name for this Spring Cloud Service. Possible values are `B0`, `S0` and `E0`. Defaults to `S0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the SKU Tier for this Spring Cloud Service. Possible values are `Basic`, `Enterprise`, `Standard` and `StandardGen2`. The attribute is automatically computed from API response except when `managed_environment_id` is defined. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `trace` block as defined below.
        #[builder(into, default)]
        pub trace: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudServiceTrace>,
        >,
        /// Whether zone redundancy is enabled for this Spring Cloud Service. Defaults to `false`.
        #[builder(into, default)]
        pub zone_redundant: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudServiceResult {
        /// Specifies the size for this Spring Cloud Service's default build agent pool. Possible values are `S1`, `S2`, `S3`, `S4` and `S5`. This field is applicable only for Spring Cloud Service with enterprise tier.
        pub build_agent_pool_size: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `config_server_git_setting` block as defined below. This field is applicable only for Spring Cloud Service with basic and standard tier.
        pub config_server_git_setting: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudServiceConfigServerGitSetting,
            >,
        >,
        /// One or more `container_registry` block as defined below. This field is applicable only for Spring Cloud Service with enterprise tier.
        pub container_registries: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::appplatform::SpringCloudServiceContainerRegistry,
                >,
            >,
        >,
        /// A `default_build_service` block as defined below. This field is applicable only for Spring Cloud Service with enterprise tier.
        pub default_build_service: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudServiceDefaultBuildService,
            >,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Should the log stream in vnet injection instance could be accessed from Internet?
        pub log_stream_public_endpoint_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The resource Id of the Managed Environment that the Spring Apps instance builds on. Can only be specified when `sku_tier` is set to `StandardGen2`.
        pub managed_environment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `marketplace` block as defined below. Can only be specified when `sku` is set to `E0`.
        pub marketplace: pulumi_gestalt_rust::Output<
            super::super::types::appplatform::SpringCloudServiceMarketplace,
        >,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network` block as defined below. Changing this forces a new resource to be created.
        pub network: pulumi_gestalt_rust::Output<
            Option<super::super::types::appplatform::SpringCloudServiceNetwork>,
        >,
        /// A list of the outbound Public IP Addresses used by this Spring Cloud Service.
        pub outbound_public_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of `required_network_traffic_rules` blocks as defined below.
        pub required_network_traffic_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::appplatform::SpringCloudServiceRequiredNetworkTrafficRule,
            >,
        >,
        /// Specifies The name of the resource group in which to create the Spring Cloud Service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Whether enable the default Service Registry. This field is applicable only for Spring Cloud Service with enterprise tier.
        pub service_registry_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Spring Cloud Service Registry.
        pub service_registry_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the SKU Name for this Spring Cloud Service. Possible values are `B0`, `S0` and `E0`. Defaults to `S0`. Changing this forces a new resource to be created.
        pub sku_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the SKU Tier for this Spring Cloud Service. Possible values are `Basic`, `Enterprise`, `Standard` and `StandardGen2`. The attribute is automatically computed from API response except when `managed_environment_id` is defined. Changing this forces a new resource to be created.
        pub sku_tier: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `trace` block as defined below.
        pub trace: pulumi_gestalt_rust::Output<
            Option<super::super::types::appplatform::SpringCloudServiceTrace>,
        >,
        /// Whether zone redundancy is enabled for this Spring Cloud Service. Defaults to `false`.
        pub zone_redundant: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudServiceArgs,
    ) -> SpringCloudServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let build_agent_pool_size_binding = args
            .build_agent_pool_size
            .get_output(context);
        let config_server_git_setting_binding = args
            .config_server_git_setting
            .get_output(context);
        let container_registries_binding = args.container_registries.get_output(context);
        let default_build_service_binding = args
            .default_build_service
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let log_stream_public_endpoint_enabled_binding = args
            .log_stream_public_endpoint_enabled
            .get_output(context);
        let managed_environment_id_binding = args
            .managed_environment_id
            .get_output(context);
        let marketplace_binding = args.marketplace.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_registry_enabled_binding = args
            .service_registry_enabled
            .get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let sku_tier_binding = args.sku_tier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let trace_binding = args.trace.get_output(context);
        let zone_redundant_binding = args.zone_redundant.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudService:SpringCloudService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildAgentPoolSize".into(),
                    value: build_agent_pool_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configServerGitSetting".into(),
                    value: config_server_git_setting_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistries".into(),
                    value: container_registries_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultBuildService".into(),
                    value: default_build_service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logStreamPublicEndpointEnabled".into(),
                    value: log_stream_public_endpoint_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedEnvironmentId".into(),
                    value: managed_environment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "marketplace".into(),
                    value: marketplace_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceRegistryEnabled".into(),
                    value: service_registry_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuTier".into(),
                    value: sku_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trace".into(),
                    value: trace_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneRedundant".into(),
                    value: zone_redundant_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudServiceResult {
            build_agent_pool_size: o.get_field("buildAgentPoolSize"),
            config_server_git_setting: o.get_field("configServerGitSetting"),
            container_registries: o.get_field("containerRegistries"),
            default_build_service: o.get_field("defaultBuildService"),
            location: o.get_field("location"),
            log_stream_public_endpoint_enabled: o
                .get_field("logStreamPublicEndpointEnabled"),
            managed_environment_id: o.get_field("managedEnvironmentId"),
            marketplace: o.get_field("marketplace"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            outbound_public_ip_addresses: o.get_field("outboundPublicIpAddresses"),
            required_network_traffic_rules: o.get_field("requiredNetworkTrafficRules"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_registry_enabled: o.get_field("serviceRegistryEnabled"),
            service_registry_id: o.get_field("serviceRegistryId"),
            sku_name: o.get_field("skuName"),
            sku_tier: o.get_field("skuTier"),
            tags: o.get_field("tags"),
            trace: o.get_field("trace"),
            zone_redundant: o.get_field("zoneRedundant"),
        }
    }
}
