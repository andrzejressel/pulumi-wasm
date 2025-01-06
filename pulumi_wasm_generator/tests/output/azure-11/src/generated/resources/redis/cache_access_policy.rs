/// Manages a Redis Cache Access Policy
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: East US
///   exampleCache:
///     type: azure:redis:Cache
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       capacity: 1
///       family: P
///       skuName: Premium
///       enableNonSslPort: false
///       redisConfiguration:
///         maxmemoryReserved: 2
///         maxmemoryDelta: 2
///         maxmemoryPolicy: allkeys-lru
///   exampleCacheAccessPolicy:
///     type: azure:redis:CacheAccessPolicy
///     name: example
///     properties:
///       name: example
///       redisCacheId: ${exampleCache.id}
///       permissions: +@read +@connection +cluster|info
/// ```
///
/// ## Import
///
/// Redis Cache Access Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/cacheAccessPolicy:CacheAccessPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redis/cache1/accessPolicies/policy1
/// ```
///
pub mod cache_access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheAccessPolicyArgs {
        /// The name of the Redis Cache Access Policy. Changing this forces a new Redis Cache Access Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Permissions that are going to be assigned to this Redis Cache Access Policy.
        #[builder(into)]
        pub permissions: pulumi_wasm_rust::Output<String>,
        /// The ID of the Redis Cache. Changing this forces a new Redis Cache Access Policy to be created.
        #[builder(into)]
        pub redis_cache_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CacheAccessPolicyResult {
        /// The name of the Redis Cache Access Policy. Changing this forces a new Redis Cache Access Policy to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Permissions that are going to be assigned to this Redis Cache Access Policy.
        pub permissions: pulumi_wasm_rust::Output<String>,
        /// The ID of the Redis Cache. Changing this forces a new Redis Cache Access Policy to be created.
        pub redis_cache_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CacheAccessPolicyArgs) -> CacheAccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let permissions_binding = args.permissions.get_inner();
        let redis_cache_id_binding = args.redis_cache_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:redis/cacheAccessPolicy:CacheAccessPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "redisCacheId".into(),
                    value: &redis_cache_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "redisCacheId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CacheAccessPolicyResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            redis_cache_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisCacheId").unwrap(),
            ),
        }
    }
}
