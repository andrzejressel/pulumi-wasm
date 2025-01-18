/// Provides an ElastiCache Replication Group resource.
///
/// For working with a [Memcached cluster](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/WhatIs.html) or a
/// [single-node Redis instance (Cluster Mode Disabled)](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/WhatIs.html),
/// see the `aws.elasticache.Cluster` resource.
///
/// > **Note:** When you change an attribute, such as `engine_version`, by
/// default the ElastiCache API applies it in the next maintenance window. Because
/// of this, this provider may report a difference in its planning phase because the
/// actual modification has not yet taken place. You can use the
/// `apply_immediately` flag to instruct the service to apply the change
/// immediately. Using `apply_immediately` can result in a brief downtime as
/// servers reboots.
/// See the AWS Documentation on
/// [Modifying an ElastiCache Cache Cluster](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.Modify.html)
/// for more information.
///
/// > **Note:** Any attribute changes that re-create the resource will be applied immediately, regardless of the value of `apply_immediately`.
///
/// > **Note:** Be aware of the terminology collision around "cluster" for `aws.elasticache.ReplicationGroup`. For example, it is possible to create a ["Cluster Mode Disabled [Redis] Cluster"](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.Create.CON.Redis.html). With "Cluster Mode Enabled", the data will be stored in shards (called "node groups"). See [Redis Cluster Configuration](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/cluster-create-determine-requirements.html#redis-cluster-configuration) for a diagram of the differences. To enable cluster mode, use a parameter group that has cluster mode enabled. The default parameter groups provided by AWS end with ".cluster.on", for example `default.redis6.x.cluster.on`.
///
/// ## Example Usage
///
/// ### Redis OSS/Valkey Cluster Mode Disabled
///
/// To create a single shard primary with single read replica:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = replication_group::create(
///         "example",
///         ReplicationGroupArgs::builder()
///             .automatic_failover_enabled(true)
///             .description("example description")
///             .node_type("cache.m4.large")
///             .num_cache_clusters(2)
///             .parameter_group_name("default.redis3.2")
///             .port(6379)
///             .preferred_cache_cluster_azs(vec!["us-west-2a", "us-west-2b",])
///             .replication_group_id("tf-rep-group-1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// You have two options for adjusting the number of replicas:
///
/// * Adjusting `num_cache_clusters` directly. This will attempt to automatically add or remove replicas, but provides no granular control (e.g., preferred availability zone, cache cluster ID) for the added or removed replicas. This also currently expects cache cluster IDs in the form of `replication_group_id-00#`.
/// * Otherwise for fine grained control of the underlying cache clusters, they can be added or removed with the `aws.elasticache.Cluster` resource and its `replication_group_id` attribute. In this situation, you will need to utilize [`ignoreChanges`](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to prevent perpetual differences with the `number_cache_cluster` attribute.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = replication_group::create(
///         "example",
///         ReplicationGroupArgs::builder()
///             .automatic_failover_enabled(true)
///             .description("example description")
///             .node_type("cache.m4.large")
///             .num_cache_clusters(2)
///             .parameter_group_name("default.redis3.2")
///             .port(6379)
///             .preferred_cache_cluster_azs(vec!["us-west-2a", "us-west-2b",])
///             .replication_group_id("tf-rep-group-1")
///             .build_struct(),
///     );
///     let replica = cluster::create(
///         "replica",
///         ClusterArgs::builder()
///             .cluster_id("tf-rep-group-1-${range.value}")
///             .replication_group_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Redis OSS/Valkey Cluster Mode Enabled
///
/// To create two shards with a primary and a single read replica each:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let baz = replication_group::create(
///         "baz",
///         ReplicationGroupArgs::builder()
///             .automatic_failover_enabled(true)
///             .description("example description")
///             .node_type("cache.t2.small")
///             .num_node_groups(2)
///             .parameter_group_name("default.redis3.2.cluster.on")
///             .port(6379)
///             .replicas_per_node_group(1)
///             .replication_group_id("tf-redis-cluster")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Redis Log Delivery configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = replication_group::create(
///         "test",
///         ReplicationGroupArgs::builder()
///             .apply_immediately(true)
///             .auto_minor_version_upgrade(false)
///             .description("test description")
///             .log_delivery_configurations(
///                 vec![
///                     ReplicationGroupLogDeliveryConfiguration::builder()
///                     .destination("${example.name}").destinationType("cloudwatch-logs")
///                     .logFormat("text").logType("slow-log").build_struct(),
///                     ReplicationGroupLogDeliveryConfiguration::builder()
///                     .destination("${exampleAwsKinesisFirehoseDeliveryStream.name}")
///                     .destinationType("kinesis-firehose").logFormat("json")
///                     .logType("engine-log").build_struct(),
///                 ],
///             )
///             .maintenance_window("tue:06:30-tue:07:30")
///             .node_type("cache.t3.small")
///             .port(6379)
///             .replication_group_id("myreplicaciongroup")
///             .snapshot_window("01:00-02:00")
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **Note:** We currently do not support passing a `primary_cluster_id` in order to create the Replication Group.
///
/// > **Note:** Automatic Failover is unavailable for Redis versions earlier than 2.8.6,
/// and unavailable on T1 node types. For T2 node types, it is only available on Redis version 3.2.4 or later with cluster mode enabled. See the [High Availability Using Replication Groups](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Replication.html) guide
/// for full details on using Replication Groups.
///
/// ### Creating a secondary replication group for a global replication group
///
/// A Global Replication Group can have up to two secondary Replication Groups in different regions. These are added to an existing Global Replication Group.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_replication_group::create(
///         "example",
///         GlobalReplicationGroupArgs::builder()
///             .global_replication_group_id_suffix("example")
///             .primary_replication_group_id("${primary.id}")
///             .build_struct(),
///     );
///     let primary = replication_group::create(
///         "primary",
///         ReplicationGroupArgs::builder()
///             .description("primary replication group")
///             .engine("redis")
///             .engine_version("5.0.6")
///             .node_type("cache.m5.large")
///             .num_cache_clusters(1)
///             .replication_group_id("example-primary")
///             .build_struct(),
///     );
///     let secondary = replication_group::create(
///         "secondary",
///         ReplicationGroupArgs::builder()
///             .description("secondary replication group")
///             .global_replication_group_id("${example.globalReplicationGroupId}")
///             .num_cache_clusters(1)
///             .replication_group_id("example-secondary")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Redis AUTH and In-Transit Encryption Enabled
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = replication_group::create(
///         "example",
///         ReplicationGroupArgs::builder()
///             .auth_token("abcdefgh1234567890")
///             .auth_token_update_strategy("ROTATE")
///             .description("example with authentication")
///             .engine_version("5.0.6")
///             .node_type("cache.t2.micro")
///             .num_cache_clusters(1)
///             .parameter_group_name("default.redis5.0")
///             .port(6379)
///             .replication_group_id("example")
///             .security_group_ids(vec!["${exampleAwsSecurityGroup.id}",])
///             .subnet_group_name("${exampleAwsElasticacheSubnetGroup.name}")
///             .transit_encryption_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// > When adding a new `auth_token` to a previously passwordless replication group, using the `ROTATE` update strategy will result in support for **both** the new token and passwordless authentication. To immediately require authorization when adding the initial token, use the `SET` strategy instead. See the [Authenticating with the Redis AUTH command](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/auth.html) guide for additional details.
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache Replication Groups using the `replication_group_id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/replicationGroup:ReplicationGroup my_replication_group replication-group-1
/// ```
pub mod replication_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationGroupArgs {
        /// Specifies whether any modifications are applied immediately, or during the next maintenance window. Default is `false`.
        #[builder(into, default)]
        pub apply_immediately: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable encryption at rest.
        /// When `engine` is `redis`, default is `false`.
        /// When `engine` is `valkey`, default is `true`.
        #[builder(into, default)]
        pub at_rest_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Password used to access a password protected server. Can be specified only if `transit_encryption_enabled = true`.
        #[builder(into, default)]
        pub auth_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Strategy to use when updating the `auth_token`. Valid values are `SET`, `ROTATE`, and `DELETE`. Defaults to `ROTATE`.
        #[builder(into, default)]
        pub auth_token_update_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether minor version engine upgrades will be applied automatically to the underlying Cache Cluster instances during the maintenance window.
        /// Only supported for engine types `"redis"` and `"valkey"` and if the engine version is 6 or higher.
        /// Defaults to `true`.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether a read-only replica will be automatically promoted to read/write primary if the existing primary fails. If enabled, `num_cache_clusters` must be greater than 1. Must be enabled for Redis (cluster mode enabled) replication groups. Defaults to `false`.
        #[builder(into, default)]
        pub automatic_failover_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether cluster mode is enabled or disabled. Valid values are `enabled` or `disabled` or `compatible`
        #[builder(into, default)]
        pub cluster_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables data tiering. Data tiering is only supported for replication groups using the r6gd node type. This parameter must be set to `true` when using r6gd nodes.
        #[builder(into, default)]
        pub data_tiering_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-created description for the replication group. Must not be empty.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// Name of the cache engine to be used for the clusters in this replication group.
        /// Valid values are `redis` or `valkey`.
        /// Default is `redis`.
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// Version number of the cache engine to be used for the cache clusters in this replication group.
        /// If the version is 7 or higher, the major and minor version should be set, e.g., `7.2`.
        /// If the version is 6, the major and minor version can be set, e.g., `6.2`,
        /// or the minor version can be unspecified which will use the latest version at creation time, e.g., `6.x`.
        /// Otherwise, specify the full version desired, e.g., `5.0.6`.
        /// The actual engine version used is returned in the attribute `engine_version_actual`, see Attribute Reference below.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of your final node group (shard) snapshot. ElastiCache creates the snapshot from the primary node in the cluster. If omitted, no final snapshot will be made.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the global replication group to which this replication group should belong. If this parameter is specified, the replication group is added to the specified global replication group as a secondary replication group; otherwise, the replication group is not part of any global replication group. If `global_replication_group_id` is set, the `num_node_groups` parameter cannot be set.
        #[builder(into, default)]
        pub global_replication_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP version to advertise in the discovery protocol. Valid values are `ipv4` or `ipv6`.
        #[builder(into, default)]
        pub ip_discovery: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the key that you wish to use if encrypting at rest. If not supplied, uses service managed encryption. Can be specified only if `at_rest_encryption_enabled = true`.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the destination and format of Redis OSS/Valkey [SLOWLOG](https://redis.io/commands/slowlog) or Redis OSS/Valkey [Engine Log](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log). See the documentation on [Amazon ElastiCache](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log). See Log Delivery Configuration below for more details.
        #[builder(into, default)]
        pub log_delivery_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::elasticache::ReplicationGroupLogDeliveryConfiguration,
                >,
            >,
        >,
        /// Specifies the weekly time range for when maintenance on the cache cluster is performed. The format is `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). The minimum maintenance window is a 60 minute period. Example: `sun:05:00-sun:09:00`
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether to enable Multi-AZ Support for the replication group.
        /// If `true`, `automatic_failover_enabled` must also be enabled.
        /// Defaults to `false`.
        #[builder(into, default)]
        pub multi_az_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IP versions for cache cluster connections. Valid values are `ipv4`, `ipv6` or `dual_stack`.
        #[builder(into, default)]
        pub network_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Instance class to be used.
        /// See AWS documentation for information on [supported node types](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/nodes-select-size.html).
        /// Required unless `global_replication_group_id` is set.
        /// Cannot be set if `global_replication_group_id` is set.
        #[builder(into, default)]
        pub node_type: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of an SNS topic to send ElastiCache notifications to. Example: `arn:aws:sns:us-east-1:012345678999:my_sns_topic`
        #[builder(into, default)]
        pub notification_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of cache clusters (primary and replicas) this replication group will have.
        /// If `automatic_failover_enabled` or `multi_az_enabled` are `true`, must be at least 2.
        /// Updates will occur before other modifications.
        /// Conflicts with `num_node_groups` and `replicas_per_node_group`.
        /// Defaults to `1`.
        #[builder(into, default)]
        pub num_cache_clusters: pulumi_wasm_rust::Output<Option<i32>>,
        /// Number of node groups (shards) for this Redis replication group.
        /// Changing this number will trigger a resizing operation before other settings modifications.
        /// Conflicts with `num_cache_clusters`.
        #[builder(into, default)]
        pub num_node_groups: pulumi_wasm_rust::Output<Option<i32>>,
        /// Name of the parameter group to associate with this replication group. If this argument is omitted, the default cache parameter group for the specified engine is used. To enable "cluster mode", i.e., data sharding, use a parameter group that has the parameter `cluster-enabled` set to true.
        #[builder(into, default)]
        pub parameter_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Port number on which each of the cache nodes will accept connections. For Memcache the default is 11211, and for Redis the default port is 6379.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of EC2 availability zones in which the replication group's cache clusters will be created. The order of the availability zones in the list is considered. The first item in the list will be the primary node. Ignored when updating.
        #[builder(into, default)]
        pub preferred_cache_cluster_azs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Number of replica nodes in each node group.
        /// Changing this number will trigger a resizing operation before other settings modifications.
        /// Valid values are 0 to 5.
        /// Conflicts with `num_cache_clusters`.
        /// Can only be set if `num_node_groups` is set.
        #[builder(into, default)]
        pub replicas_per_node_group: pulumi_wasm_rust::Output<Option<i32>>,
        /// Replication group identifier. This parameter is stored as a lowercase string.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub replication_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// IDs of one or more Amazon VPC security groups associated with this replication group. Use this parameter only when you are creating a replication group in an Amazon Virtual Private Cloud.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Names of one or more Amazon VPC security groups associated with this replication group. Use this parameter only when you are creating a replication group in an Amazon Virtual Private Cloud.
        #[builder(into, default)]
        pub security_group_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of ARNs that identify Redis RDB snapshot files stored in Amazon S3. The names object names cannot contain any commas.
        #[builder(into, default)]
        pub snapshot_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of a snapshot from which to restore data into the new node group. Changing the `snapshot_name` forces a new resource.
        #[builder(into, default)]
        pub snapshot_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of days for which ElastiCache will retain automatic cache cluster snapshots before deleting them. For example, if you set SnapshotRetentionLimit to 5, then a snapshot that was taken today will be retained for 5 days before being deleted. If the value of `snapshot_retention_limit` is set to zero (0), backups are turned off. Please note that setting a `snapshot_retention_limit` is not supported on cache.t1.micro cache nodes
        #[builder(into, default)]
        pub snapshot_retention_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// Daily time range (in UTC) during which ElastiCache will begin taking a daily snapshot of your cache cluster. The minimum snapshot window is a 60 minute period. Example: `05:00-09:00`
        #[builder(into, default)]
        pub snapshot_window: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the cache subnet group to be used for the replication group.
        #[builder(into, default)]
        pub subnet_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. Adding tags to this resource will add or overwrite any existing tags on the clusters in the replication group and not to the group itself. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to enable encryption in transit.
        /// Changing this argument with an `engine_version` < `7.0.5` will force a replacement.
        /// Engine versions prior to `7.0.5` only allow this transit encryption to be configured during creation of the replication group.
        #[builder(into, default)]
        pub transit_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A setting that enables clients to migrate to in-transit encryption with no downtime.
        /// Valid values are `preferred` and `required`.
        /// When enabling encryption on an existing replication group, this must first be set to `preferred` before setting it to `required` in a subsequent apply.
        /// See the `TransitEncryptionMode` field in the [`CreateReplicationGroup` API documentation](https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_CreateReplicationGroup.html) for additional details.
        #[builder(into, default)]
        pub transit_encryption_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// User Group ID to associate with the replication group. Only a maximum of one (1) user group ID is valid. **NOTE:** This argument _is_ a set because the AWS specification allows for multiple IDs. However, in practice, AWS only allows a maximum size of one.
        #[builder(into, default)]
        pub user_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ReplicationGroupResult {
        /// Specifies whether any modifications are applied immediately, or during the next maintenance window. Default is `false`.
        pub apply_immediately: pulumi_wasm_rust::Output<bool>,
        /// ARN of the created ElastiCache Replication Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether to enable encryption at rest.
        /// When `engine` is `redis`, default is `false`.
        /// When `engine` is `valkey`, default is `true`.
        pub at_rest_encryption_enabled: pulumi_wasm_rust::Output<bool>,
        /// Password used to access a password protected server. Can be specified only if `transit_encryption_enabled = true`.
        pub auth_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Strategy to use when updating the `auth_token`. Valid values are `SET`, `ROTATE`, and `DELETE`. Defaults to `ROTATE`.
        pub auth_token_update_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether minor version engine upgrades will be applied automatically to the underlying Cache Cluster instances during the maintenance window.
        /// Only supported for engine types `"redis"` and `"valkey"` and if the engine version is 6 or higher.
        /// Defaults to `true`.
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<bool>,
        /// Specifies whether a read-only replica will be automatically promoted to read/write primary if the existing primary fails. If enabled, `num_cache_clusters` must be greater than 1. Must be enabled for Redis (cluster mode enabled) replication groups. Defaults to `false`.
        pub automatic_failover_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates if cluster mode is enabled.
        pub cluster_enabled: pulumi_wasm_rust::Output<bool>,
        /// Specifies whether cluster mode is enabled or disabled. Valid values are `enabled` or `disabled` or `compatible`
        pub cluster_mode: pulumi_wasm_rust::Output<String>,
        /// Address of the replication group configuration endpoint when cluster mode is enabled.
        pub configuration_endpoint_address: pulumi_wasm_rust::Output<String>,
        /// Enables data tiering. Data tiering is only supported for replication groups using the r6gd node type. This parameter must be set to `true` when using r6gd nodes.
        pub data_tiering_enabled: pulumi_wasm_rust::Output<bool>,
        /// User-created description for the replication group. Must not be empty.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Name of the cache engine to be used for the clusters in this replication group.
        /// Valid values are `redis` or `valkey`.
        /// Default is `redis`.
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// Version number of the cache engine to be used for the cache clusters in this replication group.
        /// If the version is 7 or higher, the major and minor version should be set, e.g., `7.2`.
        /// If the version is 6, the major and minor version can be set, e.g., `6.2`,
        /// or the minor version can be unspecified which will use the latest version at creation time, e.g., `6.x`.
        /// Otherwise, specify the full version desired, e.g., `5.0.6`.
        /// The actual engine version used is returned in the attribute `engine_version_actual`, see Attribute Reference below.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// Because ElastiCache pulls the latest minor or patch for a version, this attribute returns the running version of the cache engine.
        pub engine_version_actual: pulumi_wasm_rust::Output<String>,
        /// The name of your final node group (shard) snapshot. ElastiCache creates the snapshot from the primary node in the cluster. If omitted, no final snapshot will be made.
        pub final_snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the global replication group to which this replication group should belong. If this parameter is specified, the replication group is added to the specified global replication group as a secondary replication group; otherwise, the replication group is not part of any global replication group. If `global_replication_group_id` is set, the `num_node_groups` parameter cannot be set.
        pub global_replication_group_id: pulumi_wasm_rust::Output<String>,
        /// The IP version to advertise in the discovery protocol. Valid values are `ipv4` or `ipv6`.
        pub ip_discovery: pulumi_wasm_rust::Output<String>,
        /// The ARN of the key that you wish to use if encrypting at rest. If not supplied, uses service managed encryption. Can be specified only if `at_rest_encryption_enabled = true`.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the destination and format of Redis OSS/Valkey [SLOWLOG](https://redis.io/commands/slowlog) or Redis OSS/Valkey [Engine Log](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log). See the documentation on [Amazon ElastiCache](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log). See Log Delivery Configuration below for more details.
        pub log_delivery_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::elasticache::ReplicationGroupLogDeliveryConfiguration,
                >,
            >,
        >,
        /// Specifies the weekly time range for when maintenance on the cache cluster is performed. The format is `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). The minimum maintenance window is a 60 minute period. Example: `sun:05:00-sun:09:00`
        pub maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Identifiers of all the nodes that are part of this replication group.
        pub member_clusters: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies whether to enable Multi-AZ Support for the replication group.
        /// If `true`, `automatic_failover_enabled` must also be enabled.
        /// Defaults to `false`.
        pub multi_az_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IP versions for cache cluster connections. Valid values are `ipv4`, `ipv6` or `dual_stack`.
        pub network_type: pulumi_wasm_rust::Output<String>,
        /// Instance class to be used.
        /// See AWS documentation for information on [supported node types](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/nodes-select-size.html).
        /// Required unless `global_replication_group_id` is set.
        /// Cannot be set if `global_replication_group_id` is set.
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// ARN of an SNS topic to send ElastiCache notifications to. Example: `arn:aws:sns:us-east-1:012345678999:my_sns_topic`
        pub notification_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of cache clusters (primary and replicas) this replication group will have.
        /// If `automatic_failover_enabled` or `multi_az_enabled` are `true`, must be at least 2.
        /// Updates will occur before other modifications.
        /// Conflicts with `num_node_groups` and `replicas_per_node_group`.
        /// Defaults to `1`.
        pub num_cache_clusters: pulumi_wasm_rust::Output<i32>,
        /// Number of node groups (shards) for this Redis replication group.
        /// Changing this number will trigger a resizing operation before other settings modifications.
        /// Conflicts with `num_cache_clusters`.
        pub num_node_groups: pulumi_wasm_rust::Output<i32>,
        /// Name of the parameter group to associate with this replication group. If this argument is omitted, the default cache parameter group for the specified engine is used. To enable "cluster mode", i.e., data sharding, use a parameter group that has the parameter `cluster-enabled` set to true.
        pub parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// Port number on which each of the cache nodes will accept connections. For Memcache the default is 11211, and for Redis the default port is 6379.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of EC2 availability zones in which the replication group's cache clusters will be created. The order of the availability zones in the list is considered. The first item in the list will be the primary node. Ignored when updating.
        pub preferred_cache_cluster_azs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// (Redis only) Address of the endpoint for the primary node in the replication group, if the cluster mode is disabled.
        pub primary_endpoint_address: pulumi_wasm_rust::Output<String>,
        /// (Redis only) Address of the endpoint for the reader node in the replication group, if the cluster mode is disabled.
        pub reader_endpoint_address: pulumi_wasm_rust::Output<String>,
        /// Number of replica nodes in each node group.
        /// Changing this number will trigger a resizing operation before other settings modifications.
        /// Valid values are 0 to 5.
        /// Conflicts with `num_cache_clusters`.
        /// Can only be set if `num_node_groups` is set.
        pub replicas_per_node_group: pulumi_wasm_rust::Output<i32>,
        /// Replication group identifier. This parameter is stored as a lowercase string.
        ///
        /// The following arguments are optional:
        pub replication_group_id: pulumi_wasm_rust::Output<String>,
        /// IDs of one or more Amazon VPC security groups associated with this replication group. Use this parameter only when you are creating a replication group in an Amazon Virtual Private Cloud.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Names of one or more Amazon VPC security groups associated with this replication group. Use this parameter only when you are creating a replication group in an Amazon Virtual Private Cloud.
        pub security_group_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of ARNs that identify Redis RDB snapshot files stored in Amazon S3. The names object names cannot contain any commas.
        pub snapshot_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of a snapshot from which to restore data into the new node group. Changing the `snapshot_name` forces a new resource.
        pub snapshot_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of days for which ElastiCache will retain automatic cache cluster snapshots before deleting them. For example, if you set SnapshotRetentionLimit to 5, then a snapshot that was taken today will be retained for 5 days before being deleted. If the value of `snapshot_retention_limit` is set to zero (0), backups are turned off. Please note that setting a `snapshot_retention_limit` is not supported on cache.t1.micro cache nodes
        pub snapshot_retention_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// Daily time range (in UTC) during which ElastiCache will begin taking a daily snapshot of your cache cluster. The minimum snapshot window is a 60 minute period. Example: `05:00-09:00`
        pub snapshot_window: pulumi_wasm_rust::Output<String>,
        /// Name of the cache subnet group to be used for the replication group.
        pub subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. Adding tags to this resource will add or overwrite any existing tags on the clusters in the replication group and not to the group itself. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether to enable encryption in transit.
        /// Changing this argument with an `engine_version` < `7.0.5` will force a replacement.
        /// Engine versions prior to `7.0.5` only allow this transit encryption to be configured during creation of the replication group.
        pub transit_encryption_enabled: pulumi_wasm_rust::Output<bool>,
        /// A setting that enables clients to migrate to in-transit encryption with no downtime.
        /// Valid values are `preferred` and `required`.
        /// When enabling encryption on an existing replication group, this must first be set to `preferred` before setting it to `required` in a subsequent apply.
        /// See the `TransitEncryptionMode` field in the [`CreateReplicationGroup` API documentation](https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_CreateReplicationGroup.html) for additional details.
        pub transit_encryption_mode: pulumi_wasm_rust::Output<String>,
        /// User Group ID to associate with the replication group. Only a maximum of one (1) user group ID is valid. **NOTE:** This argument _is_ a set because the AWS specification allows for multiple IDs. However, in practice, AWS only allows a maximum size of one.
        pub user_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ReplicationGroupArgs) -> ReplicationGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let apply_immediately_binding = args.apply_immediately.get_inner();
        let at_rest_encryption_enabled_binding = args
            .at_rest_encryption_enabled
            .get_inner();
        let auth_token_binding = args.auth_token.get_inner();
        let auth_token_update_strategy_binding = args
            .auth_token_update_strategy
            .get_inner();
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_inner();
        let automatic_failover_enabled_binding = args
            .automatic_failover_enabled
            .get_inner();
        let cluster_mode_binding = args.cluster_mode.get_inner();
        let data_tiering_enabled_binding = args.data_tiering_enabled.get_inner();
        let description_binding = args.description.get_inner();
        let engine_binding = args.engine.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let final_snapshot_identifier_binding = args
            .final_snapshot_identifier
            .get_inner();
        let global_replication_group_id_binding = args
            .global_replication_group_id
            .get_inner();
        let ip_discovery_binding = args.ip_discovery.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let log_delivery_configurations_binding = args
            .log_delivery_configurations
            .get_inner();
        let maintenance_window_binding = args.maintenance_window.get_inner();
        let multi_az_enabled_binding = args.multi_az_enabled.get_inner();
        let network_type_binding = args.network_type.get_inner();
        let node_type_binding = args.node_type.get_inner();
        let notification_topic_arn_binding = args.notification_topic_arn.get_inner();
        let num_cache_clusters_binding = args.num_cache_clusters.get_inner();
        let num_node_groups_binding = args.num_node_groups.get_inner();
        let parameter_group_name_binding = args.parameter_group_name.get_inner();
        let port_binding = args.port.get_inner();
        let preferred_cache_cluster_azs_binding = args
            .preferred_cache_cluster_azs
            .get_inner();
        let replicas_per_node_group_binding = args.replicas_per_node_group.get_inner();
        let replication_group_id_binding = args.replication_group_id.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let security_group_names_binding = args.security_group_names.get_inner();
        let snapshot_arns_binding = args.snapshot_arns.get_inner();
        let snapshot_name_binding = args.snapshot_name.get_inner();
        let snapshot_retention_limit_binding = args.snapshot_retention_limit.get_inner();
        let snapshot_window_binding = args.snapshot_window.get_inner();
        let subnet_group_name_binding = args.subnet_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_encryption_enabled_binding = args
            .transit_encryption_enabled
            .get_inner();
        let transit_encryption_mode_binding = args.transit_encryption_mode.get_inner();
        let user_group_ids_binding = args.user_group_ids.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/replicationGroup:ReplicationGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding,
                },
                register_interface::ObjectField {
                    name: "atRestEncryptionEnabled".into(),
                    value: &at_rest_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "authToken".into(),
                    value: &auth_token_binding,
                },
                register_interface::ObjectField {
                    name: "authTokenUpdateStrategy".into(),
                    value: &auth_token_update_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: &auto_minor_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "automaticFailoverEnabled".into(),
                    value: &automatic_failover_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "clusterMode".into(),
                    value: &cluster_mode_binding,
                },
                register_interface::ObjectField {
                    name: "dataTieringEnabled".into(),
                    value: &data_tiering_enabled_binding,
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
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "finalSnapshotIdentifier".into(),
                    value: &final_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "globalReplicationGroupId".into(),
                    value: &global_replication_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipDiscovery".into(),
                    value: &ip_discovery_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "logDeliveryConfigurations".into(),
                    value: &log_delivery_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "multiAzEnabled".into(),
                    value: &multi_az_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "networkType".into(),
                    value: &network_type_binding,
                },
                register_interface::ObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding,
                },
                register_interface::ObjectField {
                    name: "notificationTopicArn".into(),
                    value: &notification_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "numCacheClusters".into(),
                    value: &num_cache_clusters_binding,
                },
                register_interface::ObjectField {
                    name: "numNodeGroups".into(),
                    value: &num_node_groups_binding,
                },
                register_interface::ObjectField {
                    name: "parameterGroupName".into(),
                    value: &parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "preferredCacheClusterAzs".into(),
                    value: &preferred_cache_cluster_azs_binding,
                },
                register_interface::ObjectField {
                    name: "replicasPerNodeGroup".into(),
                    value: &replicas_per_node_group_binding,
                },
                register_interface::ObjectField {
                    name: "replicationGroupId".into(),
                    value: &replication_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupNames".into(),
                    value: &security_group_names_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotArns".into(),
                    value: &snapshot_arns_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotName".into(),
                    value: &snapshot_name_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotRetentionLimit".into(),
                    value: &snapshot_retention_limit_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotWindow".into(),
                    value: &snapshot_window_binding,
                },
                register_interface::ObjectField {
                    name: "subnetGroupName".into(),
                    value: &subnet_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitEncryptionEnabled".into(),
                    value: &transit_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "transitEncryptionMode".into(),
                    value: &transit_encryption_mode_binding,
                },
                register_interface::ObjectField {
                    name: "userGroupIds".into(),
                    value: &user_group_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applyImmediately".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "atRestEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "authToken".into(),
                },
                register_interface::ResultField {
                    name: "authTokenUpdateStrategy".into(),
                },
                register_interface::ResultField {
                    name: "autoMinorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "automaticFailoverEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clusterEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clusterMode".into(),
                },
                register_interface::ResultField {
                    name: "configurationEndpointAddress".into(),
                },
                register_interface::ResultField {
                    name: "dataTieringEnabled".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "engineVersionActual".into(),
                },
                register_interface::ResultField {
                    name: "finalSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "globalReplicationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "ipDiscovery".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "logDeliveryConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "memberClusters".into(),
                },
                register_interface::ResultField {
                    name: "multiAzEnabled".into(),
                },
                register_interface::ResultField {
                    name: "networkType".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "notificationTopicArn".into(),
                },
                register_interface::ResultField {
                    name: "numCacheClusters".into(),
                },
                register_interface::ResultField {
                    name: "numNodeGroups".into(),
                },
                register_interface::ResultField {
                    name: "parameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "preferredCacheClusterAzs".into(),
                },
                register_interface::ResultField {
                    name: "primaryEndpointAddress".into(),
                },
                register_interface::ResultField {
                    name: "readerEndpointAddress".into(),
                },
                register_interface::ResultField {
                    name: "replicasPerNodeGroup".into(),
                },
                register_interface::ResultField {
                    name: "replicationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupNames".into(),
                },
                register_interface::ResultField {
                    name: "snapshotArns".into(),
                },
                register_interface::ResultField {
                    name: "snapshotName".into(),
                },
                register_interface::ResultField {
                    name: "snapshotRetentionLimit".into(),
                },
                register_interface::ResultField {
                    name: "snapshotWindow".into(),
                },
                register_interface::ResultField {
                    name: "subnetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "transitEncryptionMode".into(),
                },
                register_interface::ResultField {
                    name: "userGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReplicationGroupResult {
            apply_immediately: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applyImmediately").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            at_rest_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("atRestEncryptionEnabled").unwrap(),
            ),
            auth_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authToken").unwrap(),
            ),
            auth_token_update_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authTokenUpdateStrategy").unwrap(),
            ),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoMinorVersionUpgrade").unwrap(),
            ),
            automatic_failover_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticFailoverEnabled").unwrap(),
            ),
            cluster_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterEnabled").unwrap(),
            ),
            cluster_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterMode").unwrap(),
            ),
            configuration_endpoint_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationEndpointAddress").unwrap(),
            ),
            data_tiering_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataTieringEnabled").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            engine_version_actual: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersionActual").unwrap(),
            ),
            final_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalSnapshotIdentifier").unwrap(),
            ),
            global_replication_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalReplicationGroupId").unwrap(),
            ),
            ip_discovery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipDiscovery").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            log_delivery_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logDeliveryConfigurations").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            member_clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memberClusters").unwrap(),
            ),
            multi_az_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAzEnabled").unwrap(),
            ),
            network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkType").unwrap(),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            notification_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationTopicArn").unwrap(),
            ),
            num_cache_clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numCacheClusters").unwrap(),
            ),
            num_node_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numNodeGroups").unwrap(),
            ),
            parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterGroupName").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            preferred_cache_cluster_azs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredCacheClusterAzs").unwrap(),
            ),
            primary_endpoint_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryEndpointAddress").unwrap(),
            ),
            reader_endpoint_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readerEndpointAddress").unwrap(),
            ),
            replicas_per_node_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicasPerNodeGroup").unwrap(),
            ),
            replication_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationGroupId").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            security_group_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupNames").unwrap(),
            ),
            snapshot_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotArns").unwrap(),
            ),
            snapshot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotName").unwrap(),
            ),
            snapshot_retention_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotRetentionLimit").unwrap(),
            ),
            snapshot_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotWindow").unwrap(),
            ),
            subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitEncryptionEnabled").unwrap(),
            ),
            transit_encryption_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitEncryptionMode").unwrap(),
            ),
            user_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userGroupIds").unwrap(),
            ),
        }
    }
}
