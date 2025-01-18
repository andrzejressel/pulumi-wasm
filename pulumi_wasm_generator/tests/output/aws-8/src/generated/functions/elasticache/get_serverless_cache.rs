pub mod get_serverless_cache {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessCacheArgs {
        /// Identifier for the serverless cache.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetServerlessCacheResult {
        /// The Amazon Resource Name (ARN) of the serverless cache.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The cache usage limits for storage and ElastiCache Processing Units for the cache. See `cache_usage_limits` Block for details.
        pub cache_usage_limits: pulumi_wasm_rust::Output<
            super::super::super::types::elasticache::GetServerlessCacheCacheUsageLimits,
        >,
        /// Timestamp of when the serverless cache was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The daily time that snapshots will be created from the new serverless cache. Only available for engine types `"redis"` and `"valkey"`.
        pub daily_snapshot_time: pulumi_wasm_rust::Output<String>,
        /// Description of the serverless cache.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Represents the information required for client programs to connect to the cache. See `endpoint` Block for details.
        pub endpoint: pulumi_wasm_rust::Output<
            super::super::super::types::elasticache::GetServerlessCacheEndpoint,
        >,
        /// Name of the cache engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// The name and version number of the engine the serverless cache is compatible with.
        pub full_engine_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the customer managed key for encrypting the data at rest.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The version number of the engine the serverless cache is compatible with.
        pub major_engine_version: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Represents the information required for client programs to connect to a cache node. See `reader_endpoint` Block for details.
        pub reader_endpoint: pulumi_wasm_rust::Output<
            super::super::super::types::elasticache::GetServerlessCacheReaderEndpoint,
        >,
        /// A list of the one or more VPC security groups associated with the serverless cache.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The number of snapshots that will be retained for the serverless cache. Available for Redis only.
        pub snapshot_retention_limit: pulumi_wasm_rust::Output<i32>,
        /// The current status of the serverless cache.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A list of the identifiers of the subnets where the VPC endpoint for the serverless cache are deployed.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The identifier of the UserGroup associated with the serverless cache. Available for Redis only.
        pub user_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServerlessCacheArgs) -> GetServerlessCacheResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticache/getServerlessCache:getServerlessCache".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cacheUsageLimits".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dailySnapshotTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "fullEngineVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "majorEngineVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "readerEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "snapshotRetentionLimit".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "userGroupId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServerlessCacheResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cache_usage_limits: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheUsageLimits").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            daily_snapshot_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailySnapshotTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            full_engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fullEngineVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            major_engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("majorEngineVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            reader_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readerEndpoint").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            snapshot_retention_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotRetentionLimit").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            user_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userGroupId").unwrap(),
            ),
        }
    }
}
