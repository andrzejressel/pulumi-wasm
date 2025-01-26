/// Provides an ElastiCache Cluster resource, which manages either a
/// [Memcached cluster](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/WhatIs.html), a
/// [single-node Redis instance](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/WhatIs.html), or a
/// [read replica in a Redis (Cluster Mode Enabled) replication group].
///
/// For working with Redis (Cluster Mode Enabled) replication groups, see the
/// `aws.elasticache.ReplicationGroup` resource.
///
/// > **Note:** When you change an attribute, such as `num_cache_nodes`, by default
/// it is applied in the next maintenance window. Because of this, this provider may report
/// a difference in its planning phase because the actual modification has not yet taken
/// place. You can use the `apply_immediately` flag to instruct the service to apply the
/// change immediately. Using `apply_immediately` can result in a brief downtime as the server reboots.
/// See the AWS Documentation on Modifying an ElastiCache Cache Cluster for
/// [ElastiCache for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/Clusters.Modify.html) or
/// [ElastiCache for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.Modify.html)
/// for more information.
///
/// > **Note:** Any attribute changes that re-create the resource will be applied immediately, regardless of the value of `apply_immediately`.
///
/// ## Example Usage
///
/// ### Memcached Cluster
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .cluster_id("cluster-example")
///             .engine("memcached")
///             .node_type("cache.m4.large")
///             .num_cache_nodes(2)
///             .parameter_group_name("default.memcached1.4")
///             .port(11211)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Redis Instance
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .cluster_id("cluster-example")
///             .engine("redis")
///             .engine_version("3.2.10")
///             .node_type("cache.m4.large")
///             .num_cache_nodes(1)
///             .parameter_group_name("default.redis3.2")
///             .port(6379)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Redis Cluster Mode Disabled Read Replica Instance
///
/// These inherit their settings from the replication group.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let replica = cluster::create(
///         "replica",
///         ClusterArgs::builder()
///             .cluster_id("cluster-example")
///             .replication_group_id("${example.id}")
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
///     let test = cluster::create(
///         "test",
///         ClusterArgs::builder()
///             .apply_immediately(true)
///             .cluster_id("mycluster")
///             .engine("redis")
///             .log_delivery_configurations(
///                 vec![
///                     ClusterLogDeliveryConfiguration::builder()
///                     .destination("${example.name}").destinationType("cloudwatch-logs")
///                     .logFormat("text").logType("slow-log").build_struct(),
///                     ClusterLogDeliveryConfiguration::builder()
///                     .destination("${exampleAwsKinesisFirehoseDeliveryStream.name}")
///                     .destinationType("kinesis-firehose").logFormat("json")
///                     .logType("engine-log").build_struct(),
///                 ],
///             )
///             .node_type("cache.t3.micro")
///             .num_cache_nodes(1)
///             .port(6379)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Elasticache Cluster in Outpost
///
/// ```yaml
/// resources:
///   exampleVpc:
///     type: aws:ec2:Vpc
///     name: example
///     properties:
///       cidrBlock: 10.0.0.0/16
///   exampleSubnet:
///     type: aws:ec2:Subnet
///     name: example
///     properties:
///       vpcId: ${exampleVpc.id}
///       cidrBlock: 10.0.1.0/24
///       tags:
///         Name: my-subnet
///   exampleSubnetGroup:
///     type: aws:elasticache:SubnetGroup
///     name: example
///     properties:
///       name: my-cache-subnet
///       subnetIds:
///         - ${exampleSubnet.id}
///   exampleCluster:
///     type: aws:elasticache:Cluster
///     name: example
///     properties:
///       clusterId: cluster-example
///       outpostMode: single-outpost
///       preferredOutpostArn: ${exampleGetOutpost.arn}
///       engine: memcached
///       nodeType: cache.r5.large
///       numCacheNodes: 2
///       parameterGroupName: default.memcached1.4
///       port: 11211
///       subnetGroupName: ${exampleSubnetGroup.name}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:outposts:getOutposts
///       arguments: {}
///   exampleGetOutpost:
///     fn::invoke:
///       function: aws:outposts:getOutpost
///       arguments:
///         id: ${example.ids[0]}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache Clusters using the `cluster_id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/cluster:Cluster my_cluster my_cluster
/// ```
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Whether any database modifications are applied immediately, or during the next maintenance window. Default is `false`. See [Amazon ElastiCache Documentation for more information.](https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_ModifyCacheCluster.html).
        #[builder(into, default)]
        pub apply_immediately: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether minor version engine upgrades will be applied automatically to the underlying Cache Cluster instances during the maintenance window.
        /// Only supported for engine type `"redis"` and if the engine version is 6 or higher.
        /// Defaults to `true`.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Availability Zone for the cache cluster. If you want to create cache nodes in multi-az, use `preferred_availability_zones` instead. Default: System chosen Availability Zone. Changing this value will re-create the resource.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the nodes in this Memcached node group are created in a single Availability Zone or created across multiple Availability Zones in the cluster's region. Valid values for this parameter are `single-az` or `cross-az`, default is `single-az`. If you want to choose `cross-az`, `num_cache_nodes` must be greater than `1`.
        #[builder(into, default)]
        pub az_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Group identifier. ElastiCache converts this name to lowercase. Changing this value will re-create the resource.
        #[builder(into, default)]
        pub cluster_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the cache engine to be used for this cache cluster. Valid values are `memcached` and `redis`.
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Version number of the cache engine to be used.
        /// If not set, defaults to the latest version.
        /// See [Describe Cache Engine Versions](https://docs.aws.amazon.com/cli/latest/reference/elasticache/describe-cache-engine-versions.html) in the AWS Documentation for supported versions.
        /// When `engine` is `redis` and the version is 7 or higher, the major and minor version should be set, e.g., `7.2`.
        /// When the version is 6, the major and minor version can be set, e.g., `6.2`,
        /// or the minor version can be unspecified which will use the latest version at creation time, e.g., `6.x`.
        /// Otherwise, specify the full version desired, e.g., `5.0.6`.
        /// The actual engine version used is returned in the attribute `engine_version_actual`, see Attribute Reference below. Cannot be provided with `replication_group_id.`
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of your final cluster snapshot. If omitted, no final snapshot will be made.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The IP version to advertise in the discovery protocol. Valid values are `ipv4` or `ipv6`.
        #[builder(into, default)]
        pub ip_discovery: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the destination and format of Redis [SLOWLOG](https://redis.io/commands/slowlog) or Redis [Engine Log](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log). See the documentation on [Amazon ElastiCache](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html). See Log Delivery Configuration below for more details.
        #[builder(into, default)]
        pub log_delivery_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::elasticache::ClusterLogDeliveryConfiguration>,
            >,
        >,
        /// Specifies the weekly time range for when maintenance
        /// on the cache cluster is performed. The format is `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC).
        /// The minimum maintenance window is a 60 minute period. Example: `sun:05:00-sun:09:00`.
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The IP versions for cache cluster connections. IPv6 is supported with Redis engine `6.2` onword or Memcached version `1.6.6` for all [Nitro system](https://aws.amazon.com/ec2/nitro/) instances. Valid values are `ipv4`, `ipv6` or `dual_stack`.
        #[builder(into, default)]
        pub network_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The instance class used.
        /// See AWS documentation for information on [supported node types for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/nodes-select-size.html).
        /// See AWS documentation for information on [supported node types for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/nodes-select-size.html).
        /// For Memcached, changing this value will re-create the resource.
        #[builder(into, default)]
        pub node_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of an SNS topic to send ElastiCache notifications to. Example: `arn:aws:sns:us-east-1:012345678999:my_sns_topic`.
        #[builder(into, default)]
        pub notification_topic_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The initial number of cache nodes that the cache cluster will have. For Redis, this value must be 1. For Memcached, this value must be between 1 and 40. If this number is reduced on subsequent runs, the highest numbered nodes will be removed.
        #[builder(into, default)]
        pub num_cache_nodes: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specify the outpost mode that will apply to the cache cluster creation. Valid values are `"single-outpost"` and `"cross-outpost"`, however AWS currently only supports `"single-outpost"` mode.
        #[builder(into, default)]
        pub outpost_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the parameter group to associate with this cache cluster.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub parameter_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The port number on which each of the cache nodes will accept connections. For Memcached the default is 11211, and for Redis the default port is 6379. Cannot be provided with `replication_group_id`. Changing this value will re-create the resource.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// List of the Availability Zones in which cache nodes are created. If you are creating your cluster in an Amazon VPC you can only locate nodes in Availability Zones that are associated with the subnets in the selected subnet group. The number of Availability Zones listed must equal the value of `num_cache_nodes`. If you want all the nodes in the same Availability Zone, use `availability_zone` instead, or repeat the Availability Zone multiple times in the list. Default: System chosen Availability Zones. Detecting drift of existing node availability zone is not currently supported. Updating this argument by itself to migrate existing node availability zones is not currently supported and will show a perpetual difference.
        #[builder(into, default)]
        pub preferred_availability_zones: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The outpost ARN in which the cache cluster will be created.
        #[builder(into, default)]
        pub preferred_outpost_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the replication group to which this cluster should belong. If this parameter is specified, the cluster is added to the specified replication group as a read replica; otherwise, the cluster is a standalone primary that is not part of any replication group.
        #[builder(into, default)]
        pub replication_group_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more VPC security groups associated with the cache cluster. Cannot be provided with `replication_group_id.`
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Single-element string list containing an Amazon Resource Name (ARN) of a Redis RDB snapshot file stored in Amazon S3. The object name cannot contain any commas. Changing `snapshot_arns` forces a new resource.
        #[builder(into, default)]
        pub snapshot_arns: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of a snapshot from which to restore data into the new node group. Changing `snapshot_name` forces a new resource.
        #[builder(into, default)]
        pub snapshot_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Number of days for which ElastiCache will retain automatic cache cluster snapshots before deleting them. For example, if you set SnapshotRetentionLimit to 5, then a snapshot that was taken today will be retained for 5 days before being deleted. If the value of SnapshotRetentionLimit is set to zero (0), backups are turned off. Please note that setting a `snapshot_retention_limit` is not supported on cache.t1.micro cache nodes
        #[builder(into, default)]
        pub snapshot_retention_limit: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Daily time range (in UTC) during which ElastiCache will begin taking a daily snapshot of your cache cluster. Example: 05:00-09:00
        #[builder(into, default)]
        pub snapshot_window: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the subnet group to be used for the cache cluster. Changing this value will re-create the resource. Cannot be provided with `replication_group_id.`
        #[builder(into, default)]
        pub subnet_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable encryption in-transit. Supported only with Memcached versions `1.6.12` and later, running in a VPC. See the [ElastiCache in-transit encryption](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/in-transit-encryption-mc.html) documentation for more details.
        #[builder(into, default)]
        pub transit_encryption_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Whether any database modifications are applied immediately, or during the next maintenance window. Default is `false`. See [Amazon ElastiCache Documentation for more information.](https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_ModifyCacheCluster.html).
        pub apply_immediately: pulumi_wasm_rust::Output<bool>,
        /// The ARN of the created ElastiCache Cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies whether minor version engine upgrades will be applied automatically to the underlying Cache Cluster instances during the maintenance window.
        /// Only supported for engine type `"redis"` and if the engine version is 6 or higher.
        /// Defaults to `true`.
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<Option<String>>,
        /// Availability Zone for the cache cluster. If you want to create cache nodes in multi-az, use `preferred_availability_zones` instead. Default: System chosen Availability Zone. Changing this value will re-create the resource.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Whether the nodes in this Memcached node group are created in a single Availability Zone or created across multiple Availability Zones in the cluster's region. Valid values for this parameter are `single-az` or `cross-az`, default is `single-az`. If you want to choose `cross-az`, `num_cache_nodes` must be greater than `1`.
        pub az_mode: pulumi_wasm_rust::Output<String>,
        /// List of node objects including `id`, `address`, `port` and `availability_zone`.
        pub cache_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::types::elasticache::ClusterCacheNode>,
        >,
        /// (Memcached only) DNS name of the cache cluster without the port appended.
        pub cluster_address: pulumi_wasm_rust::Output<String>,
        /// Group identifier. ElastiCache converts this name to lowercase. Changing this value will re-create the resource.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// (Memcached only) Configuration endpoint to allow host discovery.
        pub configuration_endpoint: pulumi_wasm_rust::Output<String>,
        /// Name of the cache engine to be used for this cache cluster. Valid values are `memcached` and `redis`.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Version number of the cache engine to be used.
        /// If not set, defaults to the latest version.
        /// See [Describe Cache Engine Versions](https://docs.aws.amazon.com/cli/latest/reference/elasticache/describe-cache-engine-versions.html) in the AWS Documentation for supported versions.
        /// When `engine` is `redis` and the version is 7 or higher, the major and minor version should be set, e.g., `7.2`.
        /// When the version is 6, the major and minor version can be set, e.g., `6.2`,
        /// or the minor version can be unspecified which will use the latest version at creation time, e.g., `6.x`.
        /// Otherwise, specify the full version desired, e.g., `5.0.6`.
        /// The actual engine version used is returned in the attribute `engine_version_actual`, see Attribute Reference below. Cannot be provided with `replication_group_id.`
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// Because ElastiCache pulls the latest minor or patch for a version, this attribute returns the running version of the cache engine.
        pub engine_version_actual: pulumi_wasm_rust::Output<String>,
        /// Name of your final cluster snapshot. If omitted, no final snapshot will be made.
        pub final_snapshot_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP version to advertise in the discovery protocol. Valid values are `ipv4` or `ipv6`.
        pub ip_discovery: pulumi_wasm_rust::Output<String>,
        /// Specifies the destination and format of Redis [SLOWLOG](https://redis.io/commands/slowlog) or Redis [Engine Log](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html#Log_contents-engine-log). See the documentation on [Amazon ElastiCache](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Log_Delivery.html). See Log Delivery Configuration below for more details.
        pub log_delivery_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::elasticache::ClusterLogDeliveryConfiguration>,
            >,
        >,
        /// Specifies the weekly time range for when maintenance
        /// on the cache cluster is performed. The format is `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC).
        /// The minimum maintenance window is a 60 minute period. Example: `sun:05:00-sun:09:00`.
        pub maintenance_window: pulumi_wasm_rust::Output<String>,
        /// The IP versions for cache cluster connections. IPv6 is supported with Redis engine `6.2` onword or Memcached version `1.6.6` for all [Nitro system](https://aws.amazon.com/ec2/nitro/) instances. Valid values are `ipv4`, `ipv6` or `dual_stack`.
        pub network_type: pulumi_wasm_rust::Output<String>,
        /// The instance class used.
        /// See AWS documentation for information on [supported node types for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/nodes-select-size.html).
        /// See AWS documentation for information on [supported node types for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/nodes-select-size.html).
        /// For Memcached, changing this value will re-create the resource.
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// ARN of an SNS topic to send ElastiCache notifications to. Example: `arn:aws:sns:us-east-1:012345678999:my_sns_topic`.
        pub notification_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The initial number of cache nodes that the cache cluster will have. For Redis, this value must be 1. For Memcached, this value must be between 1 and 40. If this number is reduced on subsequent runs, the highest numbered nodes will be removed.
        pub num_cache_nodes: pulumi_wasm_rust::Output<i32>,
        /// Specify the outpost mode that will apply to the cache cluster creation. Valid values are `"single-outpost"` and `"cross-outpost"`, however AWS currently only supports `"single-outpost"` mode.
        pub outpost_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the parameter group to associate with this cache cluster.
        ///
        /// The following arguments are optional:
        pub parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// The port number on which each of the cache nodes will accept connections. For Memcached the default is 11211, and for Redis the default port is 6379. Cannot be provided with `replication_group_id`. Changing this value will re-create the resource.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// List of the Availability Zones in which cache nodes are created. If you are creating your cluster in an Amazon VPC you can only locate nodes in Availability Zones that are associated with the subnets in the selected subnet group. The number of Availability Zones listed must equal the value of `num_cache_nodes`. If you want all the nodes in the same Availability Zone, use `availability_zone` instead, or repeat the Availability Zone multiple times in the list. Default: System chosen Availability Zones. Detecting drift of existing node availability zone is not currently supported. Updating this argument by itself to migrate existing node availability zones is not currently supported and will show a perpetual difference.
        pub preferred_availability_zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The outpost ARN in which the cache cluster will be created.
        pub preferred_outpost_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the replication group to which this cluster should belong. If this parameter is specified, the cluster is added to the specified replication group as a read replica; otherwise, the cluster is a standalone primary that is not part of any replication group.
        pub replication_group_id: pulumi_wasm_rust::Output<String>,
        /// One or more VPC security groups associated with the cache cluster. Cannot be provided with `replication_group_id.`
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Single-element string list containing an Amazon Resource Name (ARN) of a Redis RDB snapshot file stored in Amazon S3. The object name cannot contain any commas. Changing `snapshot_arns` forces a new resource.
        pub snapshot_arns: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of a snapshot from which to restore data into the new node group. Changing `snapshot_name` forces a new resource.
        pub snapshot_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of days for which ElastiCache will retain automatic cache cluster snapshots before deleting them. For example, if you set SnapshotRetentionLimit to 5, then a snapshot that was taken today will be retained for 5 days before being deleted. If the value of SnapshotRetentionLimit is set to zero (0), backups are turned off. Please note that setting a `snapshot_retention_limit` is not supported on cache.t1.micro cache nodes
        pub snapshot_retention_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// Daily time range (in UTC) during which ElastiCache will begin taking a daily snapshot of your cache cluster. Example: 05:00-09:00
        pub snapshot_window: pulumi_wasm_rust::Output<String>,
        /// Name of the subnet group to be used for the cache cluster. Changing this value will re-create the resource. Cannot be provided with `replication_group_id.`
        pub subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable encryption in-transit. Supported only with Memcached versions `1.6.12` and later, running in a VPC. See the [ElastiCache in-transit encryption](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/in-transit-encryption-mc.html) documentation for more details.
        pub transit_encryption_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let apply_immediately_binding = args
            .apply_immediately
            .get_output(context)
            .get_inner();
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_output(context)
            .get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let az_mode_binding = args.az_mode.get_output(context).get_inner();
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let final_snapshot_identifier_binding = args
            .final_snapshot_identifier
            .get_output(context)
            .get_inner();
        let ip_discovery_binding = args.ip_discovery.get_output(context).get_inner();
        let log_delivery_configurations_binding = args
            .log_delivery_configurations
            .get_output(context)
            .get_inner();
        let maintenance_window_binding = args
            .maintenance_window
            .get_output(context)
            .get_inner();
        let network_type_binding = args.network_type.get_output(context).get_inner();
        let node_type_binding = args.node_type.get_output(context).get_inner();
        let notification_topic_arn_binding = args
            .notification_topic_arn
            .get_output(context)
            .get_inner();
        let num_cache_nodes_binding = args
            .num_cache_nodes
            .get_output(context)
            .get_inner();
        let outpost_mode_binding = args.outpost_mode.get_output(context).get_inner();
        let parameter_group_name_binding = args
            .parameter_group_name
            .get_output(context)
            .get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let preferred_availability_zones_binding = args
            .preferred_availability_zones
            .get_output(context)
            .get_inner();
        let preferred_outpost_arn_binding = args
            .preferred_outpost_arn
            .get_output(context)
            .get_inner();
        let replication_group_id_binding = args
            .replication_group_id
            .get_output(context)
            .get_inner();
        let security_group_ids_binding = args
            .security_group_ids
            .get_output(context)
            .get_inner();
        let snapshot_arns_binding = args.snapshot_arns.get_output(context).get_inner();
        let snapshot_name_binding = args.snapshot_name.get_output(context).get_inner();
        let snapshot_retention_limit_binding = args
            .snapshot_retention_limit
            .get_output(context)
            .get_inner();
        let snapshot_window_binding = args
            .snapshot_window
            .get_output(context)
            .get_inner();
        let subnet_group_name_binding = args
            .subnet_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_encryption_enabled_binding = args
            .transit_encryption_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding,
                },
                register_interface::ObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: &auto_minor_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "azMode".into(),
                    value: &az_mode_binding,
                },
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
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
                    name: "ipDiscovery".into(),
                    value: &ip_discovery_binding,
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
                    name: "numCacheNodes".into(),
                    value: &num_cache_nodes_binding,
                },
                register_interface::ObjectField {
                    name: "outpostMode".into(),
                    value: &outpost_mode_binding,
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
                    name: "preferredAvailabilityZones".into(),
                    value: &preferred_availability_zones_binding,
                },
                register_interface::ObjectField {
                    name: "preferredOutpostArn".into(),
                    value: &preferred_outpost_arn_binding,
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterResult {
            apply_immediately: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applyImmediately"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoMinorVersionUpgrade"),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            az_mode: pulumi_wasm_rust::__private::into_domain(o.extract_field("azMode")),
            cache_nodes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cacheNodes"),
            ),
            cluster_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterAddress"),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            configuration_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationEndpoint"),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(o.extract_field("engine")),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            engine_version_actual: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("engineVersionActual"),
            ),
            final_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("finalSnapshotIdentifier"),
            ),
            ip_discovery: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipDiscovery"),
            ),
            log_delivery_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logDeliveryConfigurations"),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenanceWindow"),
            ),
            network_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkType"),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeType"),
            ),
            notification_topic_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notificationTopicArn"),
            ),
            num_cache_nodes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("numCacheNodes"),
            ),
            outpost_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outpostMode"),
            ),
            parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameterGroupName"),
            ),
            port: pulumi_wasm_rust::__private::into_domain(o.extract_field("port")),
            preferred_availability_zones: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preferredAvailabilityZones"),
            ),
            preferred_outpost_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preferredOutpostArn"),
            ),
            replication_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationGroupId"),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            snapshot_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotArns"),
            ),
            snapshot_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotName"),
            ),
            snapshot_retention_limit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotRetentionLimit"),
            ),
            snapshot_window: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotWindow"),
            ),
            subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            transit_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("transitEncryptionEnabled"),
            ),
        }
    }
}
