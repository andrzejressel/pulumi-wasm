/// Provides a MemoryDB Cluster.
///
/// More information about MemoryDB can be found in the [Developer Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/what-is-memorydb-for-redis.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .acl_name("open-access")
///             .engine("redis")
///             .engine_version("7.1")
///             .name("my-cluster")
///             .node_type("db.t4g.small")
///             .num_shards(2)
///             .security_group_ids(vec!["${exampleAwsSecurityGroup.id}",])
///             .snapshot_retention_limit(7)
///             .subnet_group_name("${exampleAwsMemorydbSubnetGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a cluster using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:memorydb/cluster:Cluster example my-cluster
/// ```
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// The name of the Access Control List to associate with the cluster.
        #[builder(into)]
        pub acl_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When set to `true`, the cluster will automatically receive minor engine version upgrades after launch. Defaults to `true`.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enables data tiering. This option is not supported by all instance types. For more information, see [Data tiering](https://docs.aws.amazon.com/memorydb/latest/devguide/data-tiering.html).
        #[builder(into, default)]
        pub data_tiering: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Description for the cluster. Defaults to `"Managed by Pulumi"`.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The engine that will run on your nodes. Supported values are `redis` and `valkey`.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version number of the engine to be used for the cluster. Downgrades are not supported.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the final cluster snapshot to be created when this resource is deleted. If omitted, no final snapshot will be made.
        #[builder(into, default)]
        pub final_snapshot_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the KMS key used to encrypt the cluster at rest.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the weekly time range during which maintenance on the cluster is performed. Specify as a range in the format `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). The minimum maintenance window is a 60 minute period. Example: `sun:23:00-mon:01:30`.
        #[builder(into, default)]
        pub maintenance_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The multi region cluster identifier specified on `aws.memorydb.MultiRegionCluster`.
        #[builder(into, default)]
        pub multi_region_cluster_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The compute and memory capacity of the nodes in the cluster. See AWS documentation on [supported node types](https://docs.aws.amazon.com/memorydb/latest/devguide/nodes.supportedtypes.html) as well as [vertical scaling](https://docs.aws.amazon.com/memorydb/latest/devguide/cluster-vertical-scaling.html).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub node_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of replicas to apply to each shard, up to a maximum of 5. Defaults to `1` (i.e. 2 nodes per shard).
        #[builder(into, default)]
        pub num_replicas_per_shard: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of shards in the cluster. Defaults to `1`.
        #[builder(into, default)]
        pub num_shards: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the parameter group associated with the cluster.
        #[builder(into, default)]
        pub parameter_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port number on which each of the nodes accepts connections. Defaults to `6379`.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Set of VPC Security Group ID-s to associate with this cluster.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of ARN-s that uniquely identify RDB snapshot files stored in S3. The snapshot files will be used to populate the new cluster. Object names in the ARN-s cannot contain any commas.
        #[builder(into, default)]
        pub snapshot_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of a snapshot from which to restore data into the new cluster.
        #[builder(into, default)]
        pub snapshot_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of days for which MemoryDB retains automatic snapshots before deleting them. When set to `0`, automatic backups are disabled. Defaults to `0`.
        #[builder(into, default)]
        pub snapshot_retention_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your shard. Example: `05:00-09:00`.
        #[builder(into, default)]
        pub snapshot_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the SNS topic to which cluster notifications are sent.
        #[builder(into, default)]
        pub sns_topic_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the subnet group to be used for the cluster. Defaults to a subnet group consisting of default VPC subnets.
        #[builder(into, default)]
        pub subnet_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A flag to enable in-transit encryption on the cluster. When set to `false`, the `acl_name` must be `open-access`. Defaults to `true`.
        #[builder(into, default)]
        pub tls_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The name of the Access Control List to associate with the cluster.
        pub acl_name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// When set to `true`, the cluster will automatically receive minor engine version upgrades after launch. Defaults to `true`.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        pub cluster_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memorydb::ClusterClusterEndpoint>,
        >,
        /// Enables data tiering. This option is not supported by all instance types. For more information, see [Data tiering](https://docs.aws.amazon.com/memorydb/latest/devguide/data-tiering.html).
        pub data_tiering: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Description for the cluster. Defaults to `"Managed by Pulumi"`.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The engine that will run on your nodes. Supported values are `redis` and `valkey`.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Patch version number of the engine used by the cluster.
        pub engine_patch_version: pulumi_gestalt_rust::Output<String>,
        /// Version number of the engine to be used for the cluster. Downgrades are not supported.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Name of the final cluster snapshot to be created when this resource is deleted. If omitted, no final snapshot will be made.
        pub final_snapshot_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the KMS key used to encrypt the cluster at rest.
        pub kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the weekly time range during which maintenance on the cluster is performed. Specify as a range in the format `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). The minimum maintenance window is a 60 minute period. Example: `sun:23:00-mon:01:30`.
        pub maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// The multi region cluster identifier specified on `aws.memorydb.MultiRegionCluster`.
        pub multi_region_cluster_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The compute and memory capacity of the nodes in the cluster. See AWS documentation on [supported node types](https://docs.aws.amazon.com/memorydb/latest/devguide/nodes.supportedtypes.html) as well as [vertical scaling](https://docs.aws.amazon.com/memorydb/latest/devguide/cluster-vertical-scaling.html).
        ///
        /// The following arguments are optional:
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// The number of replicas to apply to each shard, up to a maximum of 5. Defaults to `1` (i.e. 2 nodes per shard).
        pub num_replicas_per_shard: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of shards in the cluster. Defaults to `1`.
        pub num_shards: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the parameter group associated with the cluster.
        pub parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// The port number on which each of the nodes accepts connections. Defaults to `6379`.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Set of VPC Security Group ID-s to associate with this cluster.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Set of shards in this cluster.
        pub shards: pulumi_gestalt_rust::Output<
            Vec<super::super::types::memorydb::ClusterShard>,
        >,
        /// List of ARN-s that uniquely identify RDB snapshot files stored in S3. The snapshot files will be used to populate the new cluster. Object names in the ARN-s cannot contain any commas.
        pub snapshot_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of a snapshot from which to restore data into the new cluster.
        pub snapshot_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of days for which MemoryDB retains automatic snapshots before deleting them. When set to `0`, automatic backups are disabled. Defaults to `0`.
        pub snapshot_retention_limit: pulumi_gestalt_rust::Output<i32>,
        /// The daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your shard. Example: `05:00-09:00`.
        pub snapshot_window: pulumi_gestalt_rust::Output<String>,
        /// ARN of the SNS topic to which cluster notifications are sent.
        pub sns_topic_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the subnet group to be used for the cluster. Defaults to a subnet group consisting of default VPC subnets.
        pub subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A flag to enable in-transit encryption on the cluster. When set to `false`, the `acl_name` must be `open-access`. Defaults to `true`.
        pub tls_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let acl_name_binding = args.acl_name.get_output(context).get_inner();
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_output(context)
            .get_inner();
        let data_tiering_binding = args.data_tiering.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let final_snapshot_name_binding = args
            .final_snapshot_name
            .get_output(context)
            .get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_output(context).get_inner();
        let maintenance_window_binding = args
            .maintenance_window
            .get_output(context)
            .get_inner();
        let multi_region_cluster_name_binding = args
            .multi_region_cluster_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let node_type_binding = args.node_type.get_output(context).get_inner();
        let num_replicas_per_shard_binding = args
            .num_replicas_per_shard
            .get_output(context)
            .get_inner();
        let num_shards_binding = args.num_shards.get_output(context).get_inner();
        let parameter_group_name_binding = args
            .parameter_group_name
            .get_output(context)
            .get_inner();
        let port_binding = args.port.get_output(context).get_inner();
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
        let sns_topic_arn_binding = args.sns_topic_arn.get_output(context).get_inner();
        let subnet_group_name_binding = args
            .subnet_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tls_enabled_binding = args.tls_enabled.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:memorydb/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aclName".into(),
                    value: &acl_name_binding,
                },
                register_interface::ObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: &auto_minor_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "dataTiering".into(),
                    value: &data_tiering_binding,
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
                    name: "finalSnapshotName".into(),
                    value: &final_snapshot_name_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "multiRegionClusterName".into(),
                    value: &multi_region_cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding,
                },
                register_interface::ObjectField {
                    name: "numReplicasPerShard".into(),
                    value: &num_replicas_per_shard_binding,
                },
                register_interface::ObjectField {
                    name: "numShards".into(),
                    value: &num_shards_binding,
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
                    name: "snsTopicArn".into(),
                    value: &sns_topic_arn_binding,
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
                    name: "tlsEnabled".into(),
                    value: &tls_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterResult {
            acl_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aclName"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_minor_version_upgrade: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoMinorVersionUpgrade"),
            ),
            cluster_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterEndpoints"),
            ),
            data_tiering: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataTiering"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_patch_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enginePatchVersion"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            final_snapshot_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("finalSnapshotName"),
            ),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            maintenance_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceWindow"),
            ),
            multi_region_cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiRegionClusterName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            node_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeType"),
            ),
            num_replicas_per_shard: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numReplicasPerShard"),
            ),
            num_shards: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numShards"),
            ),
            parameter_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameterGroupName"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            shards: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shards"),
            ),
            snapshot_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotArns"),
            ),
            snapshot_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotName"),
            ),
            snapshot_retention_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotRetentionLimit"),
            ),
            snapshot_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotWindow"),
            ),
            sns_topic_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snsTopicArn"),
            ),
            subnet_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            tls_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsEnabled"),
            ),
        }
    }
}
