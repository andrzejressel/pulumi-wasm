#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCacheArgs {
        /// The name of the Redis cache
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group the Redis cache instance is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCacheResult {
        /// Specifies if access key authentication is enabled.
        pub access_keys_authentication_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The size of the Redis Cache deployed.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// The SKU family/pricing group used. Possible values are `C` (for Basic/Standard SKU family) and `P` (for `Premium`)
        pub family: pulumi_gestalt_rust::Output<String>,
        /// The Hostname of the Redis Instance
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The location of the Redis Cache.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The minimum TLS version.
        pub minimum_tls_version: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub non_ssl_port_enabled: pulumi_gestalt_rust::Output<bool>,
        /// A list of `patch_schedule` blocks as defined below.
        pub patch_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redis::GetCachePatchSchedule>,
        >,
        /// The non-SSL Port of the Redis Instance
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The Primary Access Key for the Redis Instance
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Redis Instance.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Static IP Address assigned to the Redis Cache when hosted inside the Virtual Network.
        pub private_static_ip_address: pulumi_gestalt_rust::Output<String>,
        /// A `redis_configuration` block as defined below.
        pub redis_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redis::GetCacheRedisConfiguration>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Access Key for the Redis Instance
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Redis Instance.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        pub shard_count: pulumi_gestalt_rust::Output<i32>,
        /// The SKU of Redis used. Possible values are `Basic`, `Standard` and `Premium`.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The SSL Port of the Redis Instance
        pub ssl_port: pulumi_gestalt_rust::Output<i32>,
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones in which this Redis Cache is located.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCacheArgs,
    ) -> GetCacheResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:redis/getCache:getCache".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCacheResult {
            access_keys_authentication_enabled: o
                .get_field("accessKeysAuthenticationEnabled"),
            capacity: o.get_field("capacity"),
            family: o.get_field("family"),
            hostname: o.get_field("hostname"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            minimum_tls_version: o.get_field("minimumTlsVersion"),
            name: o.get_field("name"),
            non_ssl_port_enabled: o.get_field("nonSslPortEnabled"),
            patch_schedules: o.get_field("patchSchedules"),
            port: o.get_field("port"),
            primary_access_key: o.get_field("primaryAccessKey"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            private_static_ip_address: o.get_field("privateStaticIpAddress"),
            redis_configurations: o.get_field("redisConfigurations"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            shard_count: o.get_field("shardCount"),
            sku_name: o.get_field("skuName"),
            ssl_port: o.get_field("sslPort"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
