/// Manages a Redis Cache.
///
/// > **Note:** Redis version 4 is being retired and no longer supports creating new instances. Version 4 will be removed in a future release. [Redis Version 4 Retirement](https://learn.microsoft.com/azure/azure-cache-for-redis/cache-retired-features#important-upgrade-timelines)
///
/// ## Example Usage
///
/// This example provisions a Standard Redis Cache.
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
pub mod cache {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheArgs {
        /// Whether access key authentication is enabled? Defaults to `true`. `active_directory_authentication_enabled` must be set to `true` to disable access key authentication.
        #[builder(into, default)]
        pub access_keys_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The size of the Redis cache to deploy. Valid values for a SKU `family` of C (Basic/Standard) are `0, 1, 2, 3, 4, 5, 6`, and for P (Premium) `family` are `1, 2, 3, 4, 5`.
        #[builder(into)]
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// The SKU family/pricing group to use. Valid values are `C` (for Basic/Standard SKU family) and `P` (for `Premium`)
        #[builder(into)]
        pub family: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::redis::CacheIdentity>,
        >,
        /// The location of the resource group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The minimum TLS version. Possible values are `1.0`, `1.1` and `1.2`. Defaults to `1.0`.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Redis instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Enable the non-SSL port (6379) - disabled by default.
        #[builder(into, default)]
        pub non_ssl_port_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of `patch_schedule` blocks as defined below.
        #[builder(into, default)]
        pub patch_schedules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::redis::CachePatchSchedule>>,
        >,
        /// The Static IP Address to assign to the Redis Cache when hosted inside the Virtual Network. This argument implies the use of `subnet_id`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub private_static_ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not public network access is allowed for this Redis Cache. `true` means this resource could be accessed by both public and private endpoint. `false` means only private endpoint access is allowed. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `redis_configuration` block as defined below - with some limitations by SKU - defaults/details are shown below.
        #[builder(into, default)]
        pub redis_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::redis::CacheRedisConfiguration>,
        >,
        /// Redis version. Only major version needed. Possible values are `4` and `6`. Defaults to `6`.
        #[builder(into, default)]
        pub redis_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount of replicas to create per master for this Redis Cache.
        ///
        /// > **Note:** Configuring the number of replicas per master is only available when using the Premium SKU and cannot be used in conjunction with shards.
        #[builder(into, default)]
        pub replicas_per_master: pulumi_wasm_rust::Output<Option<i32>>,
        /// Amount of replicas to create per primary for this Redis Cache. If both `replicas_per_primary` and `replicas_per_master` are set, they need to be equal.
        #[builder(into, default)]
        pub replicas_per_primary: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the resource group in which to create the Redis instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// *Only available when using the Premium SKU* The number of Shards to create on the Redis Cluster.
        #[builder(into, default)]
        pub shard_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The SKU of Redis to use. Possible values are `Basic`, `Standard` and `Premium`.
        ///
        /// > **Note** Downgrading the SKU will force a new resource to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// *Only available when using the Premium SKU* The ID of the Subnet within which the Redis Cache should be deployed. This Subnet must only contain Azure Cache for Redis instances without any other type of resources. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A mapping of tenant settings to assign to the resource.
        #[builder(into, default)]
        pub tenant_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Redis Cache should be located. Changing this forces a new Redis Cache to be created.
        ///
        /// > **Please Note**: Availability Zones are [in Preview and only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview) - as such you must be opted into the Preview to use this functionality. You can [opt into the Availability Zones Preview in the Azure Portal](https://aka.ms/azenroll).
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct CacheResult {
        /// Whether access key authentication is enabled? Defaults to `true`. `active_directory_authentication_enabled` must be set to `true` to disable access key authentication.
        pub access_keys_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The size of the Redis cache to deploy. Valid values for a SKU `family` of C (Basic/Standard) are `0, 1, 2, 3, 4, 5, 6`, and for P (Premium) `family` are `1, 2, 3, 4, 5`.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// The SKU family/pricing group to use. Valid values are `C` (for Basic/Standard SKU family) and `P` (for `Premium`)
        pub family: pulumi_wasm_rust::Output<String>,
        /// The Hostname of the Redis Instance
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::redis::CacheIdentity>,
        >,
        /// The location of the resource group. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The minimum TLS version. Possible values are `1.0`, `1.1` and `1.2`. Defaults to `1.0`.
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Redis instance. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Enable the non-SSL port (6379) - disabled by default.
        pub non_ssl_port_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of `patch_schedule` blocks as defined below.
        pub patch_schedules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::redis::CachePatchSchedule>>,
        >,
        /// The non-SSL Port of the Redis Instance
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The Primary Access Key for the Redis Instance
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The primary connection string of the Redis Instance.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The Static IP Address to assign to the Redis Cache when hosted inside the Virtual Network. This argument implies the use of `subnet_id`. Changing this forces a new resource to be created.
        pub private_static_ip_address: pulumi_wasm_rust::Output<String>,
        /// Whether or not public network access is allowed for this Redis Cache. `true` means this resource could be accessed by both public and private endpoint. `false` means only private endpoint access is allowed. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `redis_configuration` block as defined below - with some limitations by SKU - defaults/details are shown below.
        pub redis_configuration: pulumi_wasm_rust::Output<
            super::super::types::redis::CacheRedisConfiguration,
        >,
        /// Redis version. Only major version needed. Possible values are `4` and `6`. Defaults to `6`.
        pub redis_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount of replicas to create per master for this Redis Cache.
        ///
        /// > **Note:** Configuring the number of replicas per master is only available when using the Premium SKU and cannot be used in conjunction with shards.
        pub replicas_per_master: pulumi_wasm_rust::Output<i32>,
        /// Amount of replicas to create per primary for this Redis Cache. If both `replicas_per_primary` and `replicas_per_master` are set, they need to be equal.
        pub replicas_per_primary: pulumi_wasm_rust::Output<i32>,
        /// The name of the resource group in which to create the Redis instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Secondary Access Key for the Redis Instance
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string of the Redis Instance.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// *Only available when using the Premium SKU* The number of Shards to create on the Redis Cluster.
        pub shard_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The SKU of Redis to use. Possible values are `Basic`, `Standard` and `Premium`.
        ///
        /// > **Note** Downgrading the SKU will force a new resource to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The SSL Port of the Redis Instance
        pub ssl_port: pulumi_wasm_rust::Output<i32>,
        /// *Only available when using the Premium SKU* The ID of the Subnet within which the Redis Cache should be deployed. This Subnet must only contain Azure Cache for Redis instances without any other type of resources. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A mapping of tenant settings to assign to the resource.
        pub tenant_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Redis Cache should be located. Changing this forces a new Redis Cache to be created.
        ///
        /// > **Please Note**: Availability Zones are [in Preview and only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview) - as such you must be opted into the Preview to use this functionality. You can [opt into the Availability Zones Preview in the Azure Portal](https://aka.ms/azenroll).
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CacheArgs) -> CacheResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_keys_authentication_enabled_binding = args
            .access_keys_authentication_enabled
            .get_inner();
        let capacity_binding = args.capacity.get_inner();
        let family_binding = args.family.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let minimum_tls_version_binding = args.minimum_tls_version.get_inner();
        let name_binding = args.name.get_inner();
        let non_ssl_port_enabled_binding = args.non_ssl_port_enabled.get_inner();
        let patch_schedules_binding = args.patch_schedules.get_inner();
        let private_static_ip_address_binding = args
            .private_static_ip_address
            .get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let redis_configuration_binding = args.redis_configuration.get_inner();
        let redis_version_binding = args.redis_version.get_inner();
        let replicas_per_master_binding = args.replicas_per_master.get_inner();
        let replicas_per_primary_binding = args.replicas_per_primary.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let shard_count_binding = args.shard_count.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let tenant_settings_binding = args.tenant_settings.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:redis/cache:Cache".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessKeysAuthenticationEnabled".into(),
                    value: &access_keys_authentication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "family".into(),
                    value: &family_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minimumTlsVersion".into(),
                    value: &minimum_tls_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nonSslPortEnabled".into(),
                    value: &non_ssl_port_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "patchSchedules".into(),
                    value: &patch_schedules_binding,
                },
                register_interface::ObjectField {
                    name: "privateStaticIpAddress".into(),
                    value: &private_static_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "redisConfiguration".into(),
                    value: &redis_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "redisVersion".into(),
                    value: &redis_version_binding,
                },
                register_interface::ObjectField {
                    name: "replicasPerMaster".into(),
                    value: &replicas_per_master_binding,
                },
                register_interface::ObjectField {
                    name: "replicasPerPrimary".into(),
                    value: &replicas_per_primary_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "shardCount".into(),
                    value: &shard_count_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tenantSettings".into(),
                    value: &tenant_settings_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessKeysAuthenticationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "family".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "minimumTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nonSslPortEnabled".into(),
                },
                register_interface::ResultField {
                    name: "patchSchedules".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "privateStaticIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "redisConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "redisVersion".into(),
                },
                register_interface::ResultField {
                    name: "replicasPerMaster".into(),
                },
                register_interface::ResultField {
                    name: "replicasPerPrimary".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "shardCount".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "sslPort".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tenantSettings".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CacheResult {
            access_keys_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessKeysAuthenticationEnabled").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("family").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            minimum_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            non_ssl_port_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nonSslPortEnabled").unwrap(),
            ),
            patch_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("patchSchedules").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
            ),
            private_static_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateStaticIpAddress").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            redis_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisConfiguration").unwrap(),
            ),
            redis_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisVersion").unwrap(),
            ),
            replicas_per_master: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicasPerMaster").unwrap(),
            ),
            replicas_per_primary: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicasPerPrimary").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionString").unwrap(),
            ),
            shard_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shardCount").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            ssl_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslPort").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tenant_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantSettings").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}