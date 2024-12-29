/// Manages an RDS database instance snapshot copy. For managing RDS database cluster snapshots, see the `aws.rds.ClusterSnapshot` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance::create(
///         "example",
///         InstanceArgs::builder()
///             .allocated_storage(10)
///             .backup_retention_period(0)
///             .db_name("baz")
///             .engine("mysql")
///             .engine_version("5.6.21")
///             .instance_class("db.t2.micro")
///             .maintenance_window("Fri:09:00-Fri:09:30")
///             .parameter_group_name("default.mysql5.6")
///             .password("barbarbarbar")
///             .username("foo")
///             .build_struct(),
///     );
///     let exampleSnapshot = snapshot::create(
///         "exampleSnapshot",
///         SnapshotArgs::builder()
///             .db_instance_identifier("${example.identifier}")
///             .db_snapshot_identifier("testsnapshot1234")
///             .build_struct(),
///     );
///     let exampleSnapshotCopy = snapshot_copy::create(
///         "exampleSnapshotCopy",
///         SnapshotCopyArgs::builder()
///             .source_db_snapshot_identifier("${exampleSnapshot.dbSnapshotArn}")
///             .target_db_snapshot_identifier("testsnapshot1234-copy")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_db_snapshot_copy` using the snapshot identifier. For example:
///
/// ```sh
/// $ pulumi import aws:rds/snapshotCopy:SnapshotCopy example my-snapshot
/// ```
pub mod snapshot_copy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotCopyArgs {
        /// Whether to copy existing tags. Defaults to `false`.
        #[builder(into, default)]
        pub copy_tags: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Destination region to place snapshot copy.
        #[builder(into, default)]
        pub destination_region: pulumi_wasm_rust::Output<Option<String>>,
        /// KMS key ID.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of an option group to associate with the copy of the snapshot.
        #[builder(into, default)]
        pub option_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// he URL that contains a Signature Version 4 signed request.
        #[builder(into, default)]
        pub presigned_url: pulumi_wasm_rust::Output<Option<String>>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        #[builder(into, default)]
        pub shared_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Snapshot identifier of the source snapshot.
        #[builder(into)]
        pub source_db_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The external custom Availability Zone.
        #[builder(into, default)]
        pub target_custom_availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The Identifier for the snapshot.
        #[builder(into)]
        pub target_db_snapshot_identifier: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotCopyResult {
        /// Specifies the allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the Availability Zone the DB instance was located in at the time of the DB snapshot.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Whether to copy existing tags. Defaults to `false`.
        pub copy_tags: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) for the DB snapshot.
        pub db_snapshot_arn: pulumi_wasm_rust::Output<String>,
        /// The Destination region to place snapshot copy.
        pub destination_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the DB snapshot is encrypted.
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        /// Specifies the name of the database engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the database engine.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// Specifies the Provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.
        pub iops: pulumi_wasm_rust::Output<i32>,
        /// KMS key ID.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// License model information for the restored DB instance.
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// The name of an option group to associate with the copy of the snapshot.
        pub option_group_name: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<i32>,
        /// he URL that contains a Signature Version 4 signed request.
        pub presigned_url: pulumi_wasm_rust::Output<Option<String>>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        pub shared_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub snapshot_type: pulumi_wasm_rust::Output<String>,
        /// Snapshot identifier of the source snapshot.
        pub source_db_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// The region that the DB snapshot was created in or copied from.
        pub source_region: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage type associated with DB snapshot.
        pub storage_type: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The external custom Availability Zone.
        pub target_custom_availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The Identifier for the snapshot.
        pub target_db_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// Provides the VPC ID associated with the DB snapshot.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SnapshotCopyArgs) -> SnapshotCopyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let copy_tags_binding = args.copy_tags.get_inner();
        let destination_region_binding = args.destination_region.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let option_group_name_binding = args.option_group_name.get_inner();
        let presigned_url_binding = args.presigned_url.get_inner();
        let shared_accounts_binding = args.shared_accounts.get_inner();
        let source_db_snapshot_identifier_binding = args
            .source_db_snapshot_identifier
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let target_custom_availability_zone_binding = args
            .target_custom_availability_zone
            .get_inner();
        let target_db_snapshot_identifier_binding = args
            .target_db_snapshot_identifier
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/snapshotCopy:SnapshotCopy".into(),
            name: name.to_string(),
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
                    name: "optionGroupName".into(),
                    value: &option_group_name_binding,
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
                    name: "sourceDbSnapshotIdentifier".into(),
                    value: &source_db_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetCustomAvailabilityZone".into(),
                    value: &target_custom_availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "targetDbSnapshotIdentifier".into(),
                    value: &target_db_snapshot_identifier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocatedStorage".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "copyTags".into(),
                },
                register_interface::ResultField {
                    name: "dbSnapshotArn".into(),
                },
                register_interface::ResultField {
                    name: "destinationRegion".into(),
                },
                register_interface::ResultField {
                    name: "encrypted".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "iops".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "licenseModel".into(),
                },
                register_interface::ResultField {
                    name: "optionGroupName".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
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
                    name: "sourceDbSnapshotIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "sourceRegion".into(),
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
                    name: "targetCustomAvailabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "targetDbSnapshotIdentifier".into(),
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
        SnapshotCopyResult {
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            copy_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyTags").unwrap(),
            ),
            db_snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSnapshotArn").unwrap(),
            ),
            destination_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationRegion").unwrap(),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encrypted").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            iops: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iops").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseModel").unwrap(),
            ),
            option_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optionGroupName").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
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
            source_db_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDbSnapshotIdentifier").unwrap(),
            ),
            source_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceRegion").unwrap(),
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
            target_custom_availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetCustomAvailabilityZone").unwrap(),
            ),
            target_db_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetDbSnapshotIdentifier").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
