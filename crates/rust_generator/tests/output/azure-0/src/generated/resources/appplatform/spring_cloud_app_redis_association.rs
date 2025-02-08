/// Associates a Spring Cloud Application with a Redis Cache.
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
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: example-springcloudapp
///       resourceGroupName: ${example.name}
///       serviceName: ${exampleSpringCloudService.name}
///   exampleCache:
///     type: azure:redis:Cache
///     name: example
///     properties:
///       name: example-cache
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       capacity: 0
///       family: C
///       skuName: Basic
///       enableNonSslPort: true
///   exampleSpringCloudAppRedisAssociation:
///     type: azure:appplatform:SpringCloudAppRedisAssociation
///     name: example
///     properties:
///       name: example-bind
///       springCloudAppId: ${exampleSpringCloudApp.id}
///       redisCacheId: ${exampleCache.id}
///       redisAccessKey: ${exampleCache.primaryAccessKey}
///       sslEnabled: true
/// ```
///
/// ## Import
///
/// Spring Cloud Application Redis Association can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudAppRedisAssociation:SpringCloudAppRedisAssociation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.AppPlatform/spring/myservice/apps/myapp/bindings/bind1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod spring_cloud_app_redis_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudAppRedisAssociationArgs {
        /// Specifies the name of the Spring Cloud Application Association. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Redis Cache access key.
        #[builder(into)]
        pub redis_access_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Redis Cache resource ID. Changing this forces a new resource to be created.
        #[builder(into)]
        pub redis_cache_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Spring Cloud Application resource ID in which the Association is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should SSL be used when connecting to Redis? Defaults to `true`.
        #[builder(into, default)]
        pub ssl_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudAppRedisAssociationResult {
        /// Specifies the name of the Spring Cloud Application Association. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Redis Cache access key.
        pub redis_access_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Redis Cache resource ID. Changing this forces a new resource to be created.
        pub redis_cache_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Spring Cloud Application resource ID in which the Association is created. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
        /// Should SSL be used when connecting to Redis? Defaults to `true`.
        pub ssl_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SpringCloudAppRedisAssociationArgs,
    ) -> SpringCloudAppRedisAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let redis_access_key_binding = args
            .redis_access_key
            .get_output(context)
            .get_inner();
        let redis_cache_id_binding = args.redis_cache_id.get_output(context).get_inner();
        let spring_cloud_app_id_binding = args
            .spring_cloud_app_id
            .get_output(context)
            .get_inner();
        let ssl_enabled_binding = args.ssl_enabled.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudAppRedisAssociation:SpringCloudAppRedisAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "redisAccessKey".into(),
                    value: &redis_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "redisCacheId".into(),
                    value: &redis_cache_id_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "sslEnabled".into(),
                    value: &ssl_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudAppRedisAssociationResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            redis_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redisAccessKey"),
            ),
            redis_cache_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redisCacheId"),
            ),
            spring_cloud_app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("springCloudAppId"),
            ),
            ssl_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslEnabled"),
            ),
        }
    }
}
