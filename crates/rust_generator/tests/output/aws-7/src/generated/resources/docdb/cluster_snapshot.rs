/// Manages a DocumentDB database cluster snapshot for DocumentDB clusters.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterSnapshotArgs {
        /// The DocumentDB Cluster Identifier from which to take the snapshot.
        #[builder(into)]
        pub db_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Identifier for the snapshot.
        #[builder(into)]
        pub db_cluster_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterSnapshotResult {
        /// List of EC2 Availability Zones that instances in the DocumentDB cluster snapshot can be restored in.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The DocumentDB Cluster Identifier from which to take the snapshot.
        pub db_cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the DocumentDB Cluster Snapshot.
        pub db_cluster_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        /// The Identifier for the snapshot.
        pub db_cluster_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the database engine.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Version of the database engine for this DocumentDB cluster snapshot.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// If storage_encrypted is true, the AWS KMS key identifier for the encrypted DocumentDB cluster snapshot.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Port that the DocumentDB cluster was listening on at the time of the snapshot.
        pub port: pulumi_gestalt_rust::Output<i32>,
        pub snapshot_type: pulumi_gestalt_rust::Output<String>,
        pub source_db_cluster_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        /// The status of this DocumentDB Cluster Snapshot.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the DocumentDB cluster snapshot is encrypted.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// The VPC ID associated with the DocumentDB cluster snapshot.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterSnapshotArgs,
    ) -> ClusterSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_cluster_identifier_binding = args
            .db_cluster_identifier
            .get_output(context);
        let db_cluster_snapshot_identifier_binding = args
            .db_cluster_snapshot_identifier
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:docdb/clusterSnapshot:ClusterSnapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: &db_cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterSnapshotIdentifier".into(),
                    value: &db_cluster_snapshot_identifier_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterSnapshotResult {
            availability_zones: o.get_field("availabilityZones"),
            db_cluster_identifier: o.get_field("dbClusterIdentifier"),
            db_cluster_snapshot_arn: o.get_field("dbClusterSnapshotArn"),
            db_cluster_snapshot_identifier: o.get_field("dbClusterSnapshotIdentifier"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            kms_key_id: o.get_field("kmsKeyId"),
            port: o.get_field("port"),
            snapshot_type: o.get_field("snapshotType"),
            source_db_cluster_snapshot_arn: o.get_field("sourceDbClusterSnapshotArn"),
            status: o.get_field("status"),
            storage_encrypted: o.get_field("storageEncrypted"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
