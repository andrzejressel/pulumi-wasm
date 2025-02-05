pub mod get_serverless_cache {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessCacheArgs {
        /// Identifier for the serverless cache.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServerlessCacheArgs,
    ) -> GetServerlessCacheResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticache/getServerlessCache:getServerlessCache".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServerlessCacheResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cache_usage_limits: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cacheUsageLimits"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            daily_snapshot_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dailySnapshotTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(o.extract_field("engine")),
            full_engine_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fullEngineVersion"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            major_engine_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("majorEngineVersion"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            reader_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("readerEndpoint"),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            snapshot_retention_limit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotRetentionLimit"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            user_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userGroupId"),
            ),
        }
    }
}
