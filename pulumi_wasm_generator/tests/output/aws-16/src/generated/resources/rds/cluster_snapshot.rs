/// Manages an RDS database cluster snapshot for Aurora clusters. For managing RDS database instance snapshots, see the `aws.rds.Snapshot` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster_snapshot::create(
///         "example",
///         ClusterSnapshotArgs::builder()
///             .db_cluster_identifier("${exampleAwsRdsCluster.id}")
///             .db_cluster_snapshot_identifier("resourcetestsnapshot1234")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_db_cluster_snapshot` using the cluster snapshot identifier. For example:
///
/// ```sh
/// $ pulumi import aws:rds/clusterSnapshot:ClusterSnapshot example my-cluster-snapshot
/// ```
pub mod cluster_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterSnapshotArgs {
        /// The DB Cluster Identifier from which to take the snapshot.
        #[builder(into)]
        pub db_cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The Identifier for the snapshot.
        #[builder(into)]
        pub db_cluster_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        #[builder(into, default)]
        pub shared_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterSnapshotResult {
        /// Allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// List of EC2 Availability Zones that instances in the DB cluster snapshot can be restored in.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// The DB Cluster Identifier from which to take the snapshot.
        pub db_cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the DB Cluster Snapshot.
        pub db_cluster_snapshot_arn: pulumi_wasm_rust::Output<String>,
        /// The Identifier for the snapshot.
        pub db_cluster_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// Name of the database engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Version of the database engine for this DB cluster snapshot.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// If storage_encrypted is true, the AWS KMS key identifier for the encrypted DB cluster snapshot.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// License model information for the restored DB cluster.
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// Port that the DB cluster was listening on at the time of the snapshot.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        pub shared_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub snapshot_type: pulumi_wasm_rust::Output<String>,
        pub source_db_cluster_snapshot_arn: pulumi_wasm_rust::Output<String>,
        /// The status of this DB Cluster Snapshot.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Whether the DB cluster snapshot is encrypted.
        pub storage_encrypted: pulumi_wasm_rust::Output<bool>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPC ID associated with the DB cluster snapshot.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterSnapshotArgs) -> ClusterSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_cluster_identifier_binding = args.db_cluster_identifier.get_inner();
        let db_cluster_snapshot_identifier_binding = args
            .db_cluster_snapshot_identifier
            .get_inner();
        let shared_accounts_binding = args.shared_accounts.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/clusterSnapshot:ClusterSnapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: &db_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "dbClusterSnapshotIdentifier".into(),
                    value: &db_cluster_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "sharedAccounts".into(),
                    value: &shared_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocatedStorage".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterSnapshotArn".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "licenseModel".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccounts".into(),
                },
                register_interface::ResultField {
                    name: "snapshotType".into(),
                },
                register_interface::ResultField {
                    name: "sourceDbClusterSnapshotArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "storageEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterSnapshotResult {
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            db_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterIdentifier").unwrap(),
            ),
            db_cluster_snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterSnapshotArn").unwrap(),
            ),
            db_cluster_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterSnapshotIdentifier").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseModel").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            shared_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccounts").unwrap(),
            ),
            snapshot_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotType").unwrap(),
            ),
            source_db_cluster_snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDbClusterSnapshotArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            storage_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEncrypted").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
