pub mod get_cache {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCacheArgs {
        /// The name of the Redis cache
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group the Redis cache instance is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetCacheResult {
        /// Specifies if access key authentication is enabled.
        pub access_keys_authentication_enabled: pulumi_wasm_rust::Output<bool>,
        /// The size of the Redis Cache deployed.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// The SKU family/pricing group used. Possible values are `C` (for Basic/Standard SKU family) and `P` (for `Premium`)
        pub family: pulumi_wasm_rust::Output<String>,
        /// The Hostname of the Redis Instance
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The location of the Redis Cache.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The minimum TLS version.
        pub minimum_tls_version: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub non_ssl_port_enabled: pulumi_wasm_rust::Output<bool>,
        /// A list of `patch_schedule` blocks as defined below.
        pub patch_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redis::GetCachePatchSchedule>,
        >,
        /// The non-SSL Port of the Redis Instance
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The Primary Access Key for the Redis Instance
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The primary connection string of the Redis Instance.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The Static IP Address assigned to the Redis Cache when hosted inside the Virtual Network.
        pub private_static_ip_address: pulumi_wasm_rust::Output<String>,
        /// A `redis_configuration` block as defined below.
        pub redis_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redis::GetCacheRedisConfiguration>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Secondary Access Key for the Redis Instance
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string of the Redis Instance.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        pub shard_count: pulumi_wasm_rust::Output<i32>,
        /// The SKU of Redis used. Possible values are `Basic`, `Standard` and `Premium`.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The SSL Port of the Redis Instance
        pub ssl_port: pulumi_wasm_rust::Output<i32>,
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones in which this Redis Cache is located.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCacheArgs) -> GetCacheResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
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
                    name: "id".into(),
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
                    name: "redisConfigurations".into(),
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
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCacheResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            redis_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisConfigurations").unwrap(),
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
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
