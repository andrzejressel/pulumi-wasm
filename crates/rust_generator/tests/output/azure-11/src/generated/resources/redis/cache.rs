/// Manages a Redis Cache.
///
/// > **Note:** Redis version 4 is being retired and no longer supports creating new instances. Version 4 will be removed in a future release. [Redis Version 4 Retirement](https://learn.microsoft.com/azure/azure-cache-for-redis/cache-retired-features#important-upgrade-timelines)
///
/// ## Example Usage
///
/// This example provisions a Standard Redis Cache.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleCache = cache::create(
///         "exampleCache",
///         CacheArgs::builder()
///             .capacity(2)
///             .family("C")
///             .location("${example.location}")
///             .minimum_tls_version("1.2")
///             .name("example-cache")
///             .non_ssl_port_enabled(false)
///             .redis_configuration(CacheRedisConfiguration::builder().build_struct())
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Relevant Links
///
/// * [Azure Cache for Redis planning](https://docs.microsoft.com/azure/azure-cache-for-redis/cache-planning-faq)
/// * [Redis: Available Configuration Settings](https://redis.io/topics/config)
///
/// ## Import
///
/// Redis Cache's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/cache:Cache cache1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redis/cache1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheArgs {
        /// Whether access key authentication is enabled? Defaults to `true`. `active_directory_authentication_enabled` must be set to `true` to disable access key authentication.
        #[builder(into, default)]
        pub access_keys_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The size of the Redis cache to deploy. Valid values for a SKU `family` of C (Basic/Standard) are `0, 1, 2, 3, 4, 5, 6`, and for P (Premium) `family` are `1, 2, 3, 4, 5`.
        #[builder(into)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The SKU family/pricing group to use. Valid values are `C` (for Basic/Standard SKU family) and `P` (for `Premium`)
        #[builder(into)]
        pub family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::redis::CacheIdentity>,
        >,
        /// The location of the resource group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The minimum TLS version. Possible values are `1.0`, `1.1` and `1.2`. Defaults to `1.0`.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Redis instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable the non-SSL port (6379) - disabled by default.
        #[builder(into, default)]
        pub non_ssl_port_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of `patch_schedule` blocks as defined below.
        #[builder(into, default)]
        pub patch_schedules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::redis::CachePatchSchedule>>,
        >,
        /// The Static IP Address to assign to the Redis Cache when hosted inside the Virtual Network. This argument implies the use of `subnet_id`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub private_static_ip_address: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether or not public network access is allowed for this Redis Cache. `true` means this resource could be accessed by both public and private endpoint. `false` means only private endpoint access is allowed. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `redis_configuration` block as defined below - with some limitations by SKU - defaults/details are shown below.
        #[builder(into, default)]
        pub redis_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::redis::CacheRedisConfiguration>,
        >,
        /// Redis version. Only major version needed. Possible values are `4` and `6`. Defaults to `6`.
        #[builder(into, default)]
        pub redis_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amount of replicas to create per master for this Redis Cache.
        ///
        /// > **Note:** Configuring the number of replicas per master is only available when using the Premium SKU and cannot be used in conjunction with shards.
        #[builder(into, default)]
        pub replicas_per_master: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Amount of replicas to create per primary for this Redis Cache. If both `replicas_per_primary` and `replicas_per_master` are set, they need to be equal.
        #[builder(into, default)]
        pub replicas_per_primary: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the resource group in which to create the Redis instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// *Only available when using the Premium SKU* The number of Shards to create on the Redis Cluster.
        #[builder(into, default)]
        pub shard_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The SKU of Redis to use. Possible values are `Basic`, `Standard` and `Premium`.
        ///
        /// > **Note** Downgrading the SKU will force a new resource to be created.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// *Only available when using the Premium SKU* The ID of the Subnet within which the Redis Cache should be deployed. This Subnet must only contain Azure Cache for Redis instances without any other type of resources. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A mapping of tenant settings to assign to the resource.
        #[builder(into, default)]
        pub tenant_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Redis Cache should be located. Changing this forces a new Redis Cache to be created.
        ///
        /// > **Please Note**: Availability Zones are [in Preview and only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview) - as such you must be opted into the Preview to use this functionality. You can [opt into the Availability Zones Preview in the Azure Portal](https://aka.ms/azenroll).
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct CacheResult {
        /// Whether access key authentication is enabled? Defaults to `true`. `active_directory_authentication_enabled` must be set to `true` to disable access key authentication.
        pub access_keys_authentication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The size of the Redis cache to deploy. Valid values for a SKU `family` of C (Basic/Standard) are `0, 1, 2, 3, 4, 5, 6`, and for P (Premium) `family` are `1, 2, 3, 4, 5`.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// The SKU family/pricing group to use. Valid values are `C` (for Basic/Standard SKU family) and `P` (for `Premium`)
        pub family: pulumi_gestalt_rust::Output<String>,
        /// The Hostname of the Redis Instance
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::redis::CacheIdentity>,
        >,
        /// The location of the resource group. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The minimum TLS version. Possible values are `1.0`, `1.1` and `1.2`. Defaults to `1.0`.
        pub minimum_tls_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Redis instance. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Enable the non-SSL port (6379) - disabled by default.
        pub non_ssl_port_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of `patch_schedule` blocks as defined below.
        pub patch_schedules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::redis::CachePatchSchedule>>,
        >,
        /// The non-SSL Port of the Redis Instance
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The Primary Access Key for the Redis Instance
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Redis Instance.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Static IP Address to assign to the Redis Cache when hosted inside the Virtual Network. This argument implies the use of `subnet_id`. Changing this forces a new resource to be created.
        pub private_static_ip_address: pulumi_gestalt_rust::Output<String>,
        /// Whether or not public network access is allowed for this Redis Cache. `true` means this resource could be accessed by both public and private endpoint. `false` means only private endpoint access is allowed. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `redis_configuration` block as defined below - with some limitations by SKU - defaults/details are shown below.
        pub redis_configuration: pulumi_gestalt_rust::Output<
            super::super::types::redis::CacheRedisConfiguration,
        >,
        /// Redis version. Only major version needed. Possible values are `4` and `6`. Defaults to `6`.
        pub redis_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amount of replicas to create per master for this Redis Cache.
        ///
        /// > **Note:** Configuring the number of replicas per master is only available when using the Premium SKU and cannot be used in conjunction with shards.
        pub replicas_per_master: pulumi_gestalt_rust::Output<i32>,
        /// Amount of replicas to create per primary for this Redis Cache. If both `replicas_per_primary` and `replicas_per_master` are set, they need to be equal.
        pub replicas_per_primary: pulumi_gestalt_rust::Output<i32>,
        /// The name of the resource group in which to create the Redis instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Access Key for the Redis Instance
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Redis Instance.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// *Only available when using the Premium SKU* The number of Shards to create on the Redis Cluster.
        pub shard_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The SKU of Redis to use. Possible values are `Basic`, `Standard` and `Premium`.
        ///
        /// > **Note** Downgrading the SKU will force a new resource to be created.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The SSL Port of the Redis Instance
        pub ssl_port: pulumi_gestalt_rust::Output<i32>,
        /// *Only available when using the Premium SKU* The ID of the Subnet within which the Redis Cache should be deployed. This Subnet must only contain Azure Cache for Redis instances without any other type of resources. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A mapping of tenant settings to assign to the resource.
        pub tenant_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Redis Cache should be located. Changing this forces a new Redis Cache to be created.
        ///
        /// > **Please Note**: Availability Zones are [in Preview and only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview) - as such you must be opted into the Preview to use this functionality. You can [opt into the Availability Zones Preview in the Azure Portal](https://aka.ms/azenroll).
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CacheArgs,
    ) -> CacheResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_keys_authentication_enabled_binding = args
            .access_keys_authentication_enabled
            .get_output(context);
        let capacity_binding = args.capacity.get_output(context);
        let family_binding = args.family.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let minimum_tls_version_binding = args.minimum_tls_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let non_ssl_port_enabled_binding = args.non_ssl_port_enabled.get_output(context);
        let patch_schedules_binding = args.patch_schedules.get_output(context);
        let private_static_ip_address_binding = args
            .private_static_ip_address
            .get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let redis_configuration_binding = args.redis_configuration.get_output(context);
        let redis_version_binding = args.redis_version.get_output(context);
        let replicas_per_master_binding = args.replicas_per_master.get_output(context);
        let replicas_per_primary_binding = args.replicas_per_primary.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let shard_count_binding = args.shard_count.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tenant_settings_binding = args.tenant_settings.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:redis/cache:Cache".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessKeysAuthenticationEnabled".into(),
                    value: &access_keys_authentication_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "family".into(),
                    value: &family_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimumTlsVersion".into(),
                    value: &minimum_tls_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nonSslPortEnabled".into(),
                    value: &non_ssl_port_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "patchSchedules".into(),
                    value: &patch_schedules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateStaticIpAddress".into(),
                    value: &private_static_ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redisConfiguration".into(),
                    value: &redis_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redisVersion".into(),
                    value: &redis_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicasPerMaster".into(),
                    value: &replicas_per_master_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicasPerPrimary".into(),
                    value: &replicas_per_primary_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardCount".into(),
                    value: &shard_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantSettings".into(),
                    value: &tenant_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CacheResult {
            access_keys_authentication_enabled: o
                .get_field("accessKeysAuthenticationEnabled"),
            capacity: o.get_field("capacity"),
            family: o.get_field("family"),
            hostname: o.get_field("hostname"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            minimum_tls_version: o.get_field("minimumTlsVersion"),
            name: o.get_field("name"),
            non_ssl_port_enabled: o.get_field("nonSslPortEnabled"),
            patch_schedules: o.get_field("patchSchedules"),
            port: o.get_field("port"),
            primary_access_key: o.get_field("primaryAccessKey"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            private_static_ip_address: o.get_field("privateStaticIpAddress"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            redis_configuration: o.get_field("redisConfiguration"),
            redis_version: o.get_field("redisVersion"),
            replicas_per_master: o.get_field("replicasPerMaster"),
            replicas_per_primary: o.get_field("replicasPerPrimary"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            shard_count: o.get_field("shardCount"),
            sku_name: o.get_field("skuName"),
            ssl_port: o.get_field("sslPort"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            tenant_settings: o.get_field("tenantSettings"),
            zones: o.get_field("zones"),
        }
    }
}
