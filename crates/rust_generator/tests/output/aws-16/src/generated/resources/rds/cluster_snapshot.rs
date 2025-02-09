/// Manages an RDS database cluster snapshot for Aurora clusters. For managing RDS database instance snapshots, see the `aws.rds.Snapshot` resource.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterSnapshotArgs {
        /// The DB Cluster Identifier from which to take the snapshot.
        #[builder(into)]
        pub db_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Identifier for the snapshot.
        #[builder(into)]
        pub db_cluster_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        #[builder(into, default)]
        pub shared_accounts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterSnapshotResult {
        /// Allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// List of EC2 Availability Zones that instances in the DB cluster snapshot can be restored in.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The DB Cluster Identifier from which to take the snapshot.
        pub db_cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the DB Cluster Snapshot.
        pub db_cluster_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        /// The Identifier for the snapshot.
        pub db_cluster_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// Name of the database engine.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Version of the database engine for this DB cluster snapshot.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// If storage_encrypted is true, the AWS KMS key identifier for the encrypted DB cluster snapshot.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// License model information for the restored DB cluster.
        pub license_model: pulumi_gestalt_rust::Output<String>,
        /// Port that the DB cluster was listening on at the time of the snapshot.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        pub shared_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub snapshot_type: pulumi_gestalt_rust::Output<String>,
        pub source_db_cluster_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        /// The status of this DB Cluster Snapshot.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Whether the DB cluster snapshot is encrypted.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// A map of tags to assign to the DB cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPC ID associated with the DB cluster snapshot.
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_cluster_identifier_binding = args
            .db_cluster_identifier
            .get_output(context);
        let db_cluster_snapshot_identifier_binding = args
            .db_cluster_snapshot_identifier
            .get_output(context);
        let shared_accounts_binding = args.shared_accounts.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/clusterSnapshot:ClusterSnapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: db_cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterSnapshotIdentifier".into(),
                    value: db_cluster_snapshot_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedAccounts".into(),
                    value: shared_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterSnapshotResult {
            allocated_storage: o.get_field("allocatedStorage"),
            availability_zones: o.get_field("availabilityZones"),
            db_cluster_identifier: o.get_field("dbClusterIdentifier"),
            db_cluster_snapshot_arn: o.get_field("dbClusterSnapshotArn"),
            db_cluster_snapshot_identifier: o.get_field("dbClusterSnapshotIdentifier"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            kms_key_id: o.get_field("kmsKeyId"),
            license_model: o.get_field("licenseModel"),
            port: o.get_field("port"),
            shared_accounts: o.get_field("sharedAccounts"),
            snapshot_type: o.get_field("snapshotType"),
            source_db_cluster_snapshot_arn: o.get_field("sourceDbClusterSnapshotArn"),
            status: o.get_field("status"),
            storage_encrypted: o.get_field("storageEncrypted"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
