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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod redis_cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RedisCacheArgs {
        /// The resource ID of the API Management Service from which to create this external cache. Changing this forces a new API Management Redis Cache to be created.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location where to use cache from. Possible values are `default` and valid Azure regions. Defaults to `default`.
        #[builder(into, default)]
        pub cache_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The connection string to the Cache for Redis.
        #[builder(into)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the API Management Redis Cache.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this API Management Redis Cache. Changing this forces a new API Management Redis Cache to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Cache for Redis.
        #[builder(into, default)]
        pub redis_cache_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RedisCacheResult {
        /// The resource ID of the API Management Service from which to create this external cache. Changing this forces a new API Management Redis Cache to be created.
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The location where to use cache from. Possible values are `default` and valid Azure regions. Defaults to `default`.
        pub cache_location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The connection string to the Cache for Redis.
        pub connection_string: pulumi_gestalt_rust::Output<String>,
        /// The description of the API Management Redis Cache.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this API Management Redis Cache. Changing this forces a new API Management Redis Cache to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Cache for Redis.
        pub redis_cache_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RedisCacheArgs,
    ) -> RedisCacheResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_id_binding = args.api_management_id.get_output(context);
        let cache_location_binding = args.cache_location.get_output(context);
        let connection_string_binding = args.connection_string.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let redis_cache_id_binding = args.redis_cache_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/redisCache:RedisCache".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementId".into(),
                    value: api_management_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheLocation".into(),
                    value: cache_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionString".into(),
                    value: connection_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redisCacheId".into(),
                    value: redis_cache_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RedisCacheResult {
            api_management_id: o.get_field("apiManagementId"),
            cache_location: o.get_field("cacheLocation"),
            connection_string: o.get_field("connectionString"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            redis_cache_id: o.get_field("redisCacheId"),
        }
    }
}
