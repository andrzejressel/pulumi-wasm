#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCacheArgs,
    ) -> GetCacheResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:redis/getCache:getCache".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCacheResult {
            access_keys_authentication_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessKeysAuthenticationEnabled"),
            ),
            capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacity"),
            ),
            family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("family"),
            ),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            minimum_tls_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumTlsVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            non_ssl_port_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nonSslPortEnabled"),
            ),
            patch_schedules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("patchSchedules"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            primary_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            private_static_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateStaticIpAddress"),
            ),
            redis_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redisConfigurations"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
            secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            shard_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shardCount"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            ssl_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslPort"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            zones: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
