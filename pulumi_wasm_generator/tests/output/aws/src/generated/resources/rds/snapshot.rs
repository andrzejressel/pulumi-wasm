/// Manages an RDS database instance snapshot. For managing RDS database cluster snapshots, see the `aws.rds.ClusterSnapshot` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = instance::create(
///         "bar",
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
///     let test = snapshot::create(
///         "test",
///         SnapshotArgs::builder()
///             .db_instance_identifier("${bar.identifier}")
///             .db_snapshot_identifier("testsnapshot1234")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_db_snapshot` using the snapshot identifier. For example:
///
/// ```sh
/// $ pulumi import aws:rds/snapshot:Snapshot example my-snapshot
/// ```
pub mod snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// The DB Instance Identifier from which to take the snapshot.
        #[builder(into)]
        pub db_instance_identifier: pulumi_wasm_rust::Output<String>,
        /// The Identifier for the snapshot.
        #[builder(into)]
        pub db_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        #[builder(into, default)]
        pub shared_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// Specifies the allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the Availability Zone the DB instance was located in at the time of the DB snapshot.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The DB Instance Identifier from which to take the snapshot.
        pub db_instance_identifier: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the DB snapshot.
        pub db_snapshot_arn: pulumi_wasm_rust::Output<String>,
        /// The Identifier for the snapshot.
        pub db_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the DB snapshot is encrypted.
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        /// Specifies the name of the database engine.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the database engine.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// Specifies the Provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.
        pub iops: pulumi_wasm_rust::Output<i32>,
        /// The ARN for the KMS encryption key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// License model information for the restored DB instance.
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// Provides the option group name for the DB snapshot.
        pub option_group_name: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<i32>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        pub shared_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub snapshot_type: pulumi_wasm_rust::Output<String>,
        /// The DB snapshot Arn that the DB snapshot was copied from. It only has value in case of cross customer or cross region copy.
        pub source_db_snapshot_identifier: pulumi_wasm_rust::Output<String>,
        /// The region that the DB snapshot was created in or copied from.
        pub source_region: pulumi_wasm_rust::Output<String>,
        /// Specifies the status of this DB snapshot.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage type associated with DB snapshot.
        pub storage_type: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Provides the VPC ID associated with the DB snapshot.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SnapshotArgs) -> SnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_instance_identifier_binding = args.db_instance_identifier.get_inner();
        let db_snapshot_identifier_binding = args.db_snapshot_identifier.get_inner();
        let shared_accounts_binding = args.shared_accounts.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/snapshot:Snapshot".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbInstanceIdentifier".into(),
                    value: &db_instance_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "dbSnapshotIdentifier".into(),
                    value: &db_snapshot_identifier_binding,
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
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "dbInstanceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbSnapshotArn".into(),
                },
                register_interface::ResultField {
                    name: "dbSnapshotIdentifier".into(),
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
                    name: "status".into(),
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
        SnapshotResult {
            allocated_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedStorage").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            db_instance_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbInstanceIdentifier").unwrap(),
            ),
            db_snapshot_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSnapshotArn").unwrap(),
            ),
            db_snapshot_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbSnapshotIdentifier").unwrap(),
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
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
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
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
