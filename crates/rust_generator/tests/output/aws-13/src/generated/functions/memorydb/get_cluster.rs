pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the cluster.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags assigned to the cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// Name of the Access Control List associated with the cluster.
        pub acl_name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// True when the cluster allows automatic minor version upgrades.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<bool>,
        pub cluster_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::memorydb::GetClusterClusterEndpoint>,
        >,
        /// True when data tiering is enabled.
        pub data_tiering: pulumi_gestalt_rust::Output<bool>,
        /// Description for the cluster.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Engine that will run on cluster nodes.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Patch version number of the engine used by the cluster.
        pub engine_patch_version: pulumi_gestalt_rust::Output<String>,
        /// Version number of the engine used by the cluster.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Name of the final cluster snapshot to be created when this resource is deleted. If omitted, no final snapshot will be made.
        pub final_snapshot_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the KMS key used to encrypt the cluster at rest.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Weekly time range during which maintenance on the cluster is performed. Specify as a range in the format `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). Example: `sun:23:00-mon:01:30`.
        pub maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Name of this node.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Compute and memory capacity of the nodes in the cluster.
        pub node_type: pulumi_gestalt_rust::Output<String>,
        /// The number of replicas to apply to each shard.
        pub num_replicas_per_shard: pulumi_gestalt_rust::Output<i32>,
        /// Number of shards in the cluster.
        pub num_shards: pulumi_gestalt_rust::Output<i32>,
        /// The name of the parameter group associated with the cluster.
        pub parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// Port number that this node is listening on.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Set of VPC Security Group ID-s associated with this cluster.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of shards in this cluster.
        pub shards: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::memorydb::GetClusterShard>,
        >,
        /// The number of days for which MemoryDB retains automatic snapshots before deleting them. When set to `0`, automatic backups are disabled.
        pub snapshot_retention_limit: pulumi_gestalt_rust::Output<i32>,
        /// Daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your shard. Example: `05:00-09:00`.
        pub snapshot_window: pulumi_gestalt_rust::Output<String>,
        /// ARN of the SNS topic to which cluster notifications are sent.
        pub sns_topic_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the subnet group used for the cluster.
        pub subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the cluster.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// When true, in-transit encryption is enabled for the cluster.
        pub tls_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:memorydb/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            maintenance_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceWindow"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
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
            tls_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsEnabled"),
            ),
        }
    }
}
