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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheAccessPolicyArgs {
        /// The name of the Redis Cache Access Policy. Changing this forces a new Redis Cache Access Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Permissions that are going to be assigned to this Redis Cache Access Policy.
        #[builder(into)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Redis Cache. Changing this forces a new Redis Cache Access Policy to be created.
        #[builder(into)]
        pub redis_cache_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CacheAccessPolicyResult {
        /// The name of the Redis Cache Access Policy. Changing this forces a new Redis Cache Access Policy to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Permissions that are going to be assigned to this Redis Cache Access Policy.
        pub permissions: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Redis Cache. Changing this forces a new Redis Cache Access Policy to be created.
        pub redis_cache_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CacheAccessPolicyArgs,
    ) -> CacheAccessPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let permissions_binding = args.permissions.get_output(context);
        let redis_cache_id_binding = args.redis_cache_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:redis/cacheAccessPolicy:CacheAccessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redisCacheId".into(),
                    value: &redis_cache_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CacheAccessPolicyResult {
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            redis_cache_id: o.get_field("redisCacheId"),
        }
    }
}
