/// Manages a Redis Cache Access Policy Assignment
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
///   exampleCacheAccessPolicyAssignment:
///     type: azure:redis:CacheAccessPolicyAssignment
///     name: example
///     properties:
///       name: example
///       redisCacheId: ${exampleCache.id}
///       accessPolicyName: Data Contributor
///       objectId: ${test.objectId}
///       objectIdAlias: ServicePrincipal
/// variables:
///   test:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Redis Cache Policy Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/cacheAccessPolicyAssignment:CacheAccessPolicyAssignment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redis/cache1/accessPolicyAssignments/assignment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache_access_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheAccessPolicyAssignmentArgs {
        /// The name of the Access Policy to be assigned. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        #[builder(into)]
        pub access_policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Redis Cache Access Policy Assignment. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The principal ID to be assigned the Access Policy. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        #[builder(into)]
        pub object_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The alias of the principal ID. User-friendly name for object ID. Also represents username for token based authentication. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        #[builder(into)]
        pub object_id_alias: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Redis Cache. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        #[builder(into)]
        pub redis_cache_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CacheAccessPolicyAssignmentResult {
        /// The name of the Access Policy to be assigned. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        pub access_policy_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Redis Cache Access Policy Assignment. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The principal ID to be assigned the Access Policy. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        pub object_id: pulumi_gestalt_rust::Output<String>,
        /// The alias of the principal ID. User-friendly name for object ID. Also represents username for token based authentication. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        pub object_id_alias: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Redis Cache. Changing this forces a new Redis Cache Access Policy Assignment to be created.
        pub redis_cache_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CacheAccessPolicyAssignmentArgs,
    ) -> CacheAccessPolicyAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_policy_name_binding = args
            .access_policy_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let object_id_binding = args.object_id.get_output(context).get_inner();
        let object_id_alias_binding = args
            .object_id_alias
            .get_output(context)
            .get_inner();
        let redis_cache_id_binding = args.redis_cache_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:redis/cacheAccessPolicyAssignment:CacheAccessPolicyAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicyName".into(),
                    value: &access_policy_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "objectId".into(),
                    value: &object_id_binding,
                },
                register_interface::ObjectField {
                    name: "objectIdAlias".into(),
                    value: &object_id_alias_binding,
                },
                register_interface::ObjectField {
                    name: "redisCacheId".into(),
                    value: &redis_cache_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CacheAccessPolicyAssignmentResult {
            access_policy_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessPolicyName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            object_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectId"),
            ),
            object_id_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectIdAlias"),
            ),
            redis_cache_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redisCacheId"),
            ),
        }
    }
}
