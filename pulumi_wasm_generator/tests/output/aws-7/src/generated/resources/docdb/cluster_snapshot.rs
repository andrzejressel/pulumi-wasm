/// Manages a DocumentDB database cluster snapshot for DocumentDB clusters.
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
///             .db_cluster_identifier("${exampleAwsDocdbCluster.id}")
///             .db_cluster_snapshot_identifier("resourcetestsnapshot1234")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_docdb_cluster_snapshot` using the cluster snapshot identifier. For example:
///
/// ```sh
/// $ pulumi import aws:docdb/clusterSnapshot:ClusterSnapshot example my-cluster-snapshot
/// ```
pub mod cluster_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterSnapshotArgs {
        /// The DocumentDB Cluster Identifier from which to take the snapshot.
        #[builder(into)]
        pub db_cluster_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Identifier for the snapshot.
        #[builder(into)]
        pub db_cluster_snapshot_identifier: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterSnapshotResult {
        /// List of EC2 Availability Zones that instances in the DocumentDB cluster snapshot can be restored in.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// The DocumentDB Cluster Identifier from which to take the snapshot.
        pub db_cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the DocumentDB Cluster Snapshot.
        pub db_cluster_snapshot_arn: pulumi_wasm_rust::Output<String>,
        /// The Identifier for the snapshot.
        pub db_cluster_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the database engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Version of the database engine for this DocumentDB cluster snapshot.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// If storage_encrypted is true, the AWS KMS key identifier for the encrypted DocumentDB cluster snapshot.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Port that the DocumentDB cluster was listening on at the time of the snapshot.
        pub port: pulumi_wasm_rust::Output<i32>,
        pub snapshot_type: pulumi_wasm_rust::Output<String>,
        pub source_db_cluster_snapshot_arn: pulumi_wasm_rust::Output<String>,
        /// The status of this DocumentDB Cluster Snapshot.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the DocumentDB cluster snapshot is encrypted.
        pub storage_encrypted: pulumi_wasm_rust::Output<bool>,
        /// The VPC ID associated with the DocumentDB cluster snapshot.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterSnapshotArgs,
    ) -> ClusterSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_cluster_identifier_binding = args
            .db_cluster_identifier
            .get_output(context)
            .get_inner();
        let db_cluster_snapshot_identifier_binding = args
            .db_cluster_snapshot_identifier
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:docdb/clusterSnapshot:ClusterSnapshot".into(),
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
            ]),
            results: Vec::from([
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
                    name: "port".into(),
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
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterSnapshotResult {
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
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
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
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
