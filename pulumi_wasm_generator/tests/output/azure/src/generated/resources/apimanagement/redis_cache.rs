/// Manages a API Management Redis Cache.
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
///   exampleService:
///     type: azure:apimanagement:Service
///     name: example
///     properties:
///       name: example-apim
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       publisherName: pub1
///       publisherEmail: pub1@email.com
///       skuName: Consumption_0
///   exampleCache:
///     type: azure:redis:Cache
///     name: example
///     properties:
///       name: example-cache
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       capacity: 1
///       family: C
///       skuName: Basic
///       enableNonSslPort: false
///       minimumTlsVersion: '1.2'
///       redisConfiguration: {}
///   exampleRedisCache:
///     type: azure:apimanagement:RedisCache
///     name: example
///     properties:
///       name: example-Redis-Cache
///       apiManagementId: ${exampleService.id}
///       connectionString: ${exampleCache.primaryConnectionString}
///       description: Redis cache instances
///       redisCacheId: ${exampleCache.id}
///       cacheLocation: East Us
/// ```
///
/// ## Import
///
/// API Management Redis Caches can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/redisCache:RedisCache example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/caches/cache1
/// ```
///
pub mod redis_cache {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RedisCacheArgs {
        /// The resource ID of the API Management Service from which to create this external cache. Changing this forces a new API Management Redis Cache to be created.
        #[builder(into)]
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The location where to use cache from. Possible values are `default` and valid Azure regions. Defaults to `default`.
        #[builder(into, default)]
        pub cache_location: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection string to the Cache for Redis.
        #[builder(into)]
        pub connection_string: pulumi_wasm_rust::Output<String>,
        /// The description of the API Management Redis Cache.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this API Management Redis Cache. Changing this forces a new API Management Redis Cache to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the Cache for Redis.
        #[builder(into, default)]
        pub redis_cache_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RedisCacheResult {
        /// The resource ID of the API Management Service from which to create this external cache. Changing this forces a new API Management Redis Cache to be created.
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The location where to use cache from. Possible values are `default` and valid Azure regions. Defaults to `default`.
        pub cache_location: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection string to the Cache for Redis.
        pub connection_string: pulumi_wasm_rust::Output<String>,
        /// The description of the API Management Redis Cache.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this API Management Redis Cache. Changing this forces a new API Management Redis Cache to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Cache for Redis.
        pub redis_cache_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RedisCacheArgs) -> RedisCacheResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args.api_management_id.get_inner();
        let cache_location_binding = args.cache_location.get_inner();
        let connection_string_binding = args.connection_string.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let redis_cache_id_binding = args.redis_cache_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/redisCache:RedisCache".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "cacheLocation".into(),
                    value: &cache_location_binding,
                },
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "redisCacheId".into(),
                    value: &redis_cache_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementId".into(),
                },
                register_interface::ResultField {
                    name: "cacheLocation".into(),
                },
                register_interface::ResultField {
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        RedisCacheResult {
            api_management_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementId").unwrap(),
            ),
            cache_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheLocation").unwrap(),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            redis_cache_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisCacheId").unwrap(),
            ),
        }
    }
}