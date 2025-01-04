pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the cluster.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the cluster.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// Name of the Access Control List associated with the cluster.
        pub acl_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// True when the cluster allows automatic minor version upgrades.
        pub auto_minor_version_upgrade: pulumi_wasm_rust::Output<bool>,
        pub cluster_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::memorydb::GetClusterClusterEndpoint>,
        >,
        /// True when data tiering is enabled.
        pub data_tiering: pulumi_wasm_rust::Output<bool>,
        /// Description for the cluster.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Engine that will run on cluster nodes.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Patch version number of the engine used by the cluster.
        pub engine_patch_version: pulumi_wasm_rust::Output<String>,
        /// Version number of the engine used by the cluster.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// Name of the final cluster snapshot to be created when this resource is deleted. If omitted, no final snapshot will be made.
        pub final_snapshot_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the KMS key used to encrypt the cluster at rest.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Weekly time range during which maintenance on the cluster is performed. Specify as a range in the format `ddd:hh24:mi-ddd:hh24:mi` (24H Clock UTC). Example: `sun:23:00-mon:01:30`.
        pub maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Name of this node.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Compute and memory capacity of the nodes in the cluster.
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// The number of replicas to apply to each shard.
        pub num_replicas_per_shard: pulumi_wasm_rust::Output<i32>,
        /// Number of shards in the cluster.
        pub num_shards: pulumi_wasm_rust::Output<i32>,
        /// The name of the parameter group associated with the cluster.
        pub parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// Port number that this node is listening on.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Set of VPC Security Group ID-s associated with this cluster.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of shards in this cluster.
        pub shards: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::memorydb::GetClusterShard>,
        >,
        /// The number of days for which MemoryDB retains automatic snapshots before deleting them. When set to `0`, automatic backups are disabled.
        pub snapshot_retention_limit: pulumi_wasm_rust::Output<i32>,
        /// Daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your shard. Example: `05:00-09:00`.
        pub snapshot_window: pulumi_wasm_rust::Output<String>,
        /// ARN of the SNS topic to which cluster notifications are sent.
        pub sns_topic_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the subnet group used for the cluster.
        pub subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the cluster.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// When true, in-transit encryption is enabled for the cluster.
        pub tls_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetClusterArgs) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:memorydb/getCluster:getCluster".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "aclName".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoMinorVersionUpgrade".into(),
                },
                register_interface::ResultField {
                    name: "clusterEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "dataTiering".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "enginePatchVersion".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "finalSnapshotName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "numReplicasPerShard".into(),
                },
                register_interface::ResultField {
                    name: "numShards".into(),
                },
                register_interface::ResultField {
                    name: "parameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "shards".into(),
                },
                register_interface::ResultField {
                    name: "snapshotRetentionLimit".into(),
                },
                register_interface::ResultField {
                    name: "snapshotWindow".into(),
                },
                register_interface::ResultField {
                    name: "snsTopicArn".into(),
                },
                register_interface::ResultField {
                    name: "subnetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tlsEnabled".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            acl_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aclName").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_minor_version_upgrade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoMinorVersionUpgrade").unwrap(),
            ),
            cluster_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterEndpoints").unwrap(),
            ),
            data_tiering: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataTiering").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_patch_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enginePatchVersion").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            final_snapshot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalSnapshotName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            num_replicas_per_shard: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numReplicasPerShard").unwrap(),
            ),
            num_shards: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numShards").unwrap(),
            ),
            parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterGroupName").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            shards: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shards").unwrap(),
            ),
            snapshot_retention_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotRetentionLimit").unwrap(),
            ),
            snapshot_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotWindow").unwrap(),
            ),
            sns_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snsTopicArn").unwrap(),
            ),
            subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tls_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsEnabled").unwrap(),
            ),
        }
    }
}
