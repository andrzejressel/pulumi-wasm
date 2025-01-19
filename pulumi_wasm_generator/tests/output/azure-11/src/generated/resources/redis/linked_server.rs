/// Manages a Redis Linked Server (ie Geo Location)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example-primary:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources-primary
///       location: East US
///   example-primaryCache:
///     type: azure:redis:Cache
///     name: example-primary
///     properties:
///       name: example-cache1
///       location: ${["example-primary"].location}
///       resourceGroupName: ${["example-primary"].name}
///       capacity: 1
///       family: P
///       skuName: Premium
///       enableNonSslPort: false
///       redisConfiguration:
///         maxmemoryReserved: 2
///         maxmemoryDelta: 2
///         maxmemoryPolicy: allkeys-lru
///   example-secondary:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources-secondary
///       location: West US
///   example-secondaryCache:
///     type: azure:redis:Cache
///     name: example-secondary
///     properties:
///       name: example-cache2
///       location: ${["example-secondary"].location}
///       resourceGroupName: ${["example-secondary"].name}
///       capacity: 1
///       family: P
///       skuName: Premium
///       enableNonSslPort: false
///       redisConfiguration:
///         maxmemoryReserved: 2
///         maxmemoryDelta: 2
///         maxmemoryPolicy: allkeys-lru
///   example-link:
///     type: azure:redis:LinkedServer
///     properties:
///       targetRedisCacheName: ${["example-primaryCache"].name}
///       resourceGroupName: ${["example-primaryCache"].resourceGroupName}
///       linkedRedisCacheId: ${["example-secondaryCache"].id}
///       linkedRedisCacheLocation: ${["example-secondaryCache"].location}
///       serverRole: Secondary
/// ```
///
/// ## Import
///
/// Redis can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/linkedServer:LinkedServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redis/cache1/linkedServers/cache2
/// ```
///
pub mod linked_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServerArgs {
        /// The ID of the linked Redis cache. Changing this forces a new Redis to be created.
        #[builder(into)]
        pub linked_redis_cache_id: pulumi_wasm_rust::Output<String>,
        /// The location of the linked Redis cache. Changing this forces a new Redis to be created.
        #[builder(into)]
        pub linked_redis_cache_location: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Redis caches exists. Changing this forces a new Redis to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The role of the linked Redis cache (eg "Secondary"). Changing this forces a new Redis to be created. Possible values are `Primary` and `Secondary`.
        #[builder(into)]
        pub server_role: pulumi_wasm_rust::Output<String>,
        /// The name of Redis cache to link with. Changing this forces a new Redis to be created. (eg The primary role)
        #[builder(into)]
        pub target_redis_cache_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedServerResult {
        /// The geo-replicated primary hostname for this linked server.
        pub geo_replicated_primary_host_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the linked Redis cache. Changing this forces a new Redis to be created.
        pub linked_redis_cache_id: pulumi_wasm_rust::Output<String>,
        /// The location of the linked Redis cache. Changing this forces a new Redis to be created.
        pub linked_redis_cache_location: pulumi_wasm_rust::Output<String>,
        /// The name of the linked server.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Redis caches exists. Changing this forces a new Redis to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The role of the linked Redis cache (eg "Secondary"). Changing this forces a new Redis to be created. Possible values are `Primary` and `Secondary`.
        pub server_role: pulumi_wasm_rust::Output<String>,
        /// The name of Redis cache to link with. Changing this forces a new Redis to be created. (eg The primary role)
        pub target_redis_cache_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LinkedServerArgs) -> LinkedServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let linked_redis_cache_id_binding = args.linked_redis_cache_id.get_inner();
        let linked_redis_cache_location_binding = args
            .linked_redis_cache_location
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_role_binding = args.server_role.get_inner();
        let target_redis_cache_name_binding = args.target_redis_cache_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:redis/linkedServer:LinkedServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "linkedRedisCacheId".into(),
                    value: &linked_redis_cache_id_binding,
                },
                register_interface::ObjectField {
                    name: "linkedRedisCacheLocation".into(),
                    value: &linked_redis_cache_location_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serverRole".into(),
                    value: &server_role_binding,
                },
                register_interface::ObjectField {
                    name: "targetRedisCacheName".into(),
                    value: &target_redis_cache_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "geoReplicatedPrimaryHostName".into(),
                },
                register_interface::ResultField {
                    name: "linkedRedisCacheId".into(),
                },
                register_interface::ResultField {
                    name: "linkedRedisCacheLocation".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serverRole".into(),
                },
                register_interface::ResultField {
                    name: "targetRedisCacheName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkedServerResult {
            geo_replicated_primary_host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("geoReplicatedPrimaryHostName").unwrap(),
            ),
            linked_redis_cache_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedRedisCacheId").unwrap(),
            ),
            linked_redis_cache_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedRedisCacheLocation").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            server_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverRole").unwrap(),
            ),
            target_redis_cache_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetRedisCacheName").unwrap(),
            ),
        }
    }
}
