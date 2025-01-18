/// Manages an RDS database cluster snapshot copy. For managing RDS database instance snapshot copies, see the `aws.rds.SnapshotCopy` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod cluster_snapshot_copy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterSnapshotCopyArgs {
        /// Whether to copy existing tags. Defaults to `false`.
        #[builder(into, default)]
        pub copy_tags: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Destination region to place snapshot copy.
        #[builder(into, default)]
        pub destination_region: pulumi_wasm_rust::Output<Option<String>>,
        /// KMS key ID.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// URL that contains a Signature Version 4 signed request.
        #[builder(into, default)]
        pub presigned_url: pulumi_wasm_rust::Output<Option<String>>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        #[builder(into, default)]
        pub shared_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Identifier of the source snapshot.
        #[builder(into)]
        pub source_db_cluster_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier for the snapshot.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target_db_cluster_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::rds::ClusterSnapshotCopyTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterSnapshotCopyResult {
        /// Specifies the allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// Whether to copy existing tags. Defaults to `false`.
        pub copy_tags: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) for the DB cluster snapshot.
        pub db_cluster_snapshot_arn: pulumi_wasm_rust::Output<String>,
        /// The Destination region to place snapshot copy.
        pub destination_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the database engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the database engine.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// KMS key ID.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// License model information for the restored DB instance.
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// URL that contains a Signature Version 4 signed request.
        pub presigned_url: pulumi_wasm_rust::Output<Option<String>>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        pub shared_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub snapshot_type: pulumi_wasm_rust::Output<String>,
        /// Identifier of the source snapshot.
        pub source_db_cluster_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the DB cluster snapshot is encrypted.
        pub storage_encrypted: pulumi_wasm_rust::Output<bool>,
        /// Specifies the storage type associated with DB cluster snapshot.
        pub storage_type: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier for the snapshot.
        ///
        /// The following arguments are optional:
        pub target_db_cluster_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::rds::ClusterSnapshotCopyTimeouts>,
        >,
        /// Provides the VPC ID associated with the DB cluster snapshot.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ClusterSnapshotCopyArgs,
    ) -> ClusterSnapshotCopyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let copy_tags_binding = args.copy_tags.get_inner();
        let destination_region_binding = args.destination_region.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let presigned_url_binding = args.presigned_url.get_inner();
        let shared_accounts_binding = args.shared_accounts.get_inner();
        let source_db_cluster_snapshot_identifier_binding = args
            .source_db_cluster_snapshot_identifier
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let target_db_cluster_snapshot_identifier_binding = args
            .target_db_cluster_snapshot_identifier
            .get_inner();
        let timeouts_binding = args.timeouts.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocatedStorage".into(),
                },
                register_interface::ResultField {
                    name: "copyTags".into(),
                },
                register_interface::ResultField {
                    name: "dbClusterSnapshotArn".into(),
                },
                register_interface::ResultField {
                    name: "destinationRegion".into(),
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
                    name: "presignedUrl".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccounts".into(),
                },
                register_interface::ResultField {
                    name: "snapshotType".into(),
                },
                register_interface::ResultField {
                    name: "sourceDbClusterSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "storageEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "storageType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetDbClusterSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
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
        ClusterSnapshotCopyResult {
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            copy_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyTags").unwrap(),
            ),
            db_cluster_snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterSnapshotArn").unwrap(),
            ),
            destination_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationRegion").unwrap(),
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
            presigned_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("presignedUrl").unwrap(),
            ),
            shared_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccounts").unwrap(),
            ),
            snapshot_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotType").unwrap(),
            ),
            source_db_cluster_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDbClusterSnapshotIdentifier").unwrap(),
            ),
            storage_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEncrypted").unwrap(),
            ),
            storage_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_db_cluster_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetDbClusterSnapshotIdentifier").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
