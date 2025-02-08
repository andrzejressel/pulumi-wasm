/// Manages an RDS database cluster snapshot copy. For managing RDS database instance snapshot copies, see the `aws.rds.SnapshotCopy` resource.
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
///             .cluster_identifier("aurora-cluster-demo")
///             .database_name("test")
///             .engine("aurora-mysql")
///             .master_password("avoid-plaintext-passwords")
///             .master_username("tfacctest")
///             .skip_final_snapshot(true)
///             .build_struct(),
///     );
///     let exampleClusterSnapshot = cluster_snapshot::create(
///         "exampleClusterSnapshot",
///         ClusterSnapshotArgs::builder()
///             .db_cluster_identifier("${example.clusterIdentifier}")
///             .db_cluster_snapshot_identifier("example")
///             .build_struct(),
///     );
///     let exampleClusterSnapshotCopy = cluster_snapshot_copy::create(
///         "exampleClusterSnapshotCopy",
///         ClusterSnapshotCopyArgs::builder()
///             .source_db_cluster_snapshot_identifier(
///                 "${exampleClusterSnapshot.dbClusterSnapshotArn}",
///             )
///             .target_db_cluster_snapshot_identifier("example-copy")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_rds_cluster_snapshot_copy` using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/clusterSnapshotCopy:ClusterSnapshotCopy example my-snapshot
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_snapshot_copy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterSnapshotCopyArgs {
        /// Whether to copy existing tags. Defaults to `false`.
        #[builder(into, default)]
        pub copy_tags: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Destination region to place snapshot copy.
        #[builder(into, default)]
        pub destination_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// KMS key ID.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL that contains a Signature Version 4 signed request.
        #[builder(into, default)]
        pub presigned_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        #[builder(into, default)]
        pub shared_accounts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Identifier of the source snapshot.
        #[builder(into)]
        pub source_db_cluster_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier for the snapshot.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target_db_cluster_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::ClusterSnapshotCopyTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterSnapshotCopyResult {
        /// Specifies the allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Whether to copy existing tags. Defaults to `false`.
        pub copy_tags: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) for the DB cluster snapshot.
        pub db_cluster_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        /// The Destination region to place snapshot copy.
        pub destination_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the database engine.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Specifies the version of the database engine.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// KMS key ID.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// License model information for the restored DB instance.
        pub license_model: pulumi_gestalt_rust::Output<String>,
        /// URL that contains a Signature Version 4 signed request.
        pub presigned_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        pub shared_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub snapshot_type: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the source snapshot.
        pub source_db_cluster_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the DB cluster snapshot is encrypted.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the storage type associated with DB cluster snapshot.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier for the snapshot.
        ///
        /// The following arguments are optional:
        pub target_db_cluster_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::ClusterSnapshotCopyTimeouts>,
        >,
        /// Provides the VPC ID associated with the DB cluster snapshot.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterSnapshotCopyArgs,
    ) -> ClusterSnapshotCopyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let copy_tags_binding = args.copy_tags.get_output(context).get_inner();
        let destination_region_binding = args
            .destination_region
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let presigned_url_binding = args.presigned_url.get_output(context).get_inner();
        let shared_accounts_binding = args
            .shared_accounts
            .get_output(context)
            .get_inner();
        let source_db_cluster_snapshot_identifier_binding = args
            .source_db_cluster_snapshot_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_db_cluster_snapshot_identifier_binding = args
            .target_db_cluster_snapshot_identifier
            .get_output(context)
            .get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/clusterSnapshotCopy:ClusterSnapshotCopy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "copyTags".into(),
                    value: &copy_tags_binding,
                },
                register_interface::ObjectField {
                    name: "destinationRegion".into(),
                    value: &destination_region_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "presignedUrl".into(),
                    value: &presigned_url_binding,
                },
                register_interface::ObjectField {
                    name: "sharedAccounts".into(),
                    value: &shared_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "sourceDbClusterSnapshotIdentifier".into(),
                    value: &source_db_cluster_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetDbClusterSnapshotIdentifier".into(),
                    value: &target_db_cluster_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterSnapshotCopyResult {
            allocated_storage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocatedStorage"),
            ),
            copy_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("copyTags"),
            ),
            db_cluster_snapshot_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbClusterSnapshotArn"),
            ),
            destination_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationRegion"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            license_model: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseModel"),
            ),
            presigned_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("presignedUrl"),
            ),
            shared_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sharedAccounts"),
            ),
            snapshot_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotType"),
            ),
            source_db_cluster_snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDbClusterSnapshotIdentifier"),
            ),
            storage_encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageEncrypted"),
            ),
            storage_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_db_cluster_snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetDbClusterSnapshotIdentifier"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
