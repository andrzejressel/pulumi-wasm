/// Provides an ElastiCache Serverless Cache resource which manages memcached, redis or valkey.
///
/// ## Example Usage
///
/// ### Memcached Serverless
///
///
/// ### Redis OSS Serverless
///
///
/// ### Valkey Serverless
///
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache Serverless Cache using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/serverlessCache:ServerlessCache my_cluster my_cluster
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod serverless_cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessCacheArgs {
        /// Sets the cache usage limits for storage and ElastiCache Processing Units for the cache. See `cache_usage_limits` Block for details.
        #[builder(into, default)]
        pub cache_usage_limits: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticache::ServerlessCacheCacheUsageLimits>,
        >,
        /// The daily time that snapshots will be created from the new serverless cache. Only supported for engine types `"redis"` or `"valkey"`. Defaults to `0`.
        #[builder(into, default)]
        pub daily_snapshot_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-provided description for the serverless cache. The default is NULL.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the cache engine to be used for this cache cluster. Valid values are `memcached`, `redis` or `valkey`.
        #[builder(into)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the customer managed key for encrypting the data at rest. If no KMS key is provided, a default service key is used.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version of the cache engine that will be used to create the serverless cache.
        /// See [Describe Cache Engine Versions](https://docs.aws.amazon.com/cli/latest/reference/elasticache/describe-cache-engine-versions.html) in the AWS Documentation for supported versions.
        #[builder(into, default)]
        pub major_engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Cluster name which serves as a unique identifier to the serverless cache
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of the one or more VPC security groups to be associated with the serverless cache. The security group will authorize traffic access for the VPC end-point (private-link). If no other information is given this will be the VPC’s Default Security Group that is associated with the cluster VPC end-point.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The list of ARN(s) of the snapshot that the new serverless cache will be created from. Available for Redis only.
        #[builder(into, default)]
        pub snapshot_arns_to_restores: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The number of snapshots that will be retained for the serverless cache that is being created. As new snapshots beyond this limit are added, the oldest snapshots will be deleted on a rolling basis. Available for Redis only.
        #[builder(into, default)]
        pub snapshot_retention_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A list of the identifiers of the subnets where the VPC endpoint for the serverless cache will be deployed. All the subnetIds must belong to the same VPC.
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticache::ServerlessCacheTimeouts>,
        >,
        /// The identifier of the UserGroup to be associated with the serverless cache. Available for Redis only. Default is NULL.
        #[builder(into, default)]
        pub user_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServerlessCacheResult {
        /// The Amazon Resource Name (ARN) of the serverless cache.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Sets the cache usage limits for storage and ElastiCache Processing Units for the cache. See `cache_usage_limits` Block for details.
        pub cache_usage_limits: pulumi_gestalt_rust::Output<
            Option<super::super::types::elasticache::ServerlessCacheCacheUsageLimits>,
        >,
        /// Timestamp of when the serverless cache was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The daily time that snapshots will be created from the new serverless cache. Only supported for engine types `"redis"` or `"valkey"`. Defaults to `0`.
        pub daily_snapshot_time: pulumi_gestalt_rust::Output<String>,
        /// User-provided description for the serverless cache. The default is NULL.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Represents the information required for client programs to connect to a cache node. See `endpoint` Block for details.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::elasticache::ServerlessCacheEndpoint>,
        >,
        /// Name of the cache engine to be used for this cache cluster. Valid values are `memcached`, `redis` or `valkey`.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// The name and version number of the engine the serverless cache is compatible with.
        pub full_engine_version: pulumi_gestalt_rust::Output<String>,
        /// ARN of the customer managed key for encrypting the data at rest. If no KMS key is provided, a default service key is used.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version of the cache engine that will be used to create the serverless cache.
        /// See [Describe Cache Engine Versions](https://docs.aws.amazon.com/cli/latest/reference/elasticache/describe-cache-engine-versions.html) in the AWS Documentation for supported versions.
        pub major_engine_version: pulumi_gestalt_rust::Output<String>,
        /// The Cluster name which serves as a unique identifier to the serverless cache
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Represents the information required for client programs to connect to a cache node. See `reader_endpoint` Block for details.
        pub reader_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::elasticache::ServerlessCacheReaderEndpoint>,
        >,
        /// A list of the one or more VPC security groups to be associated with the serverless cache. The security group will authorize traffic access for the VPC end-point (private-link). If no other information is given this will be the VPC’s Default Security Group that is associated with the cluster VPC end-point.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The list of ARN(s) of the snapshot that the new serverless cache will be created from. Available for Redis only.
        pub snapshot_arns_to_restores: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The number of snapshots that will be retained for the serverless cache that is being created. As new snapshots beyond this limit are added, the oldest snapshots will be deleted on a rolling basis. Available for Redis only.
        pub snapshot_retention_limit: pulumi_gestalt_rust::Output<i32>,
        /// The current status of the serverless cache. The allowed values are CREATING, AVAILABLE, DELETING, CREATE-FAILED and MODIFYING.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A list of the identifiers of the subnets where the VPC endpoint for the serverless cache will be deployed. All the subnetIds must belong to the same VPC.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::elasticache::ServerlessCacheTimeouts>,
        >,
        /// The identifier of the UserGroup to be associated with the serverless cache. Available for Redis only. Default is NULL.
        pub user_group_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServerlessCacheArgs,
    ) -> ServerlessCacheResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cache_usage_limits_binding = args
            .cache_usage_limits
            .get_output(context)
            .get_inner();
        let daily_snapshot_time_binding = args
            .daily_snapshot_time
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let major_engine_version_binding = args
            .major_engine_version
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let security_group_ids_binding = args
            .security_group_ids
            .get_output(context)
            .get_inner();
        let snapshot_arns_to_restores_binding = args
            .snapshot_arns_to_restores
            .get_output(context)
            .get_inner();
        let snapshot_retention_limit_binding = args
            .snapshot_retention_limit
            .get_output(context)
            .get_inner();
        let subnet_ids_binding = args.subnet_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let user_group_id_binding = args.user_group_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/serverlessCache:ServerlessCache".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cacheUsageLimits".into(),
                    value: &cache_usage_limits_binding,
                },
                register_interface::ObjectField {
                    name: "dailySnapshotTime".into(),
                    value: &daily_snapshot_time_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "majorEngineVersion".into(),
                    value: &major_engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotArnsToRestores".into(),
                    value: &snapshot_arns_to_restores_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotRetentionLimit".into(),
                    value: &snapshot_retention_limit_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "userGroupId".into(),
                    value: &user_group_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerlessCacheResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cache_usage_limits: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheUsageLimits"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            daily_snapshot_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dailySnapshotTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoints"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            full_engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fullEngineVersion"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            major_engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("majorEngineVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            reader_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readerEndpoints"),
            ),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            snapshot_arns_to_restores: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotArnsToRestores"),
            ),
            snapshot_retention_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotRetentionLimit"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            user_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userGroupId"),
            ),
        }
    }
}
