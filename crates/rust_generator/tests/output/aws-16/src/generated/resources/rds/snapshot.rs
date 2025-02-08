/// Manages an RDS database instance snapshot. For managing RDS database cluster snapshots, see the `aws.rds.ClusterSnapshot` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// The DB Instance Identifier from which to take the snapshot.
        #[builder(into)]
        pub db_instance_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Identifier for the snapshot.
        #[builder(into)]
        pub db_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        #[builder(into, default)]
        pub shared_accounts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// Specifies the allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the Availability Zone the DB instance was located in at the time of the DB snapshot.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The DB Instance Identifier from which to take the snapshot.
        pub db_instance_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the DB snapshot.
        pub db_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        /// The Identifier for the snapshot.
        pub db_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the DB snapshot is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the name of the database engine.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Specifies the version of the database engine.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.
        pub iops: pulumi_gestalt_rust::Output<i32>,
        /// The ARN for the KMS encryption key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// License model information for the restored DB instance.
        pub license_model: pulumi_gestalt_rust::Output<String>,
        /// Provides the option group name for the DB snapshot.
        pub option_group_name: pulumi_gestalt_rust::Output<String>,
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        pub shared_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub snapshot_type: pulumi_gestalt_rust::Output<String>,
        /// The DB snapshot Arn that the DB snapshot was copied from. It only has value in case of cross customer or cross region copy.
        pub source_db_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// The region that the DB snapshot was created in or copied from.
        pub source_region: pulumi_gestalt_rust::Output<String>,
        /// Specifies the status of this DB snapshot.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage type associated with DB snapshot.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Provides the VPC ID associated with the DB snapshot.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SnapshotArgs,
    ) -> SnapshotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let db_instance_identifier_binding = args
            .db_instance_identifier
            .get_output(context)
            .get_inner();
        let db_snapshot_identifier_binding = args
            .db_snapshot_identifier
            .get_output(context)
            .get_inner();
        let shared_accounts_binding = args
            .shared_accounts
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SnapshotResult {
            allocated_storage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocatedStorage"),
            ),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            db_instance_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbInstanceIdentifier"),
            ),
            db_snapshot_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbSnapshotArn"),
            ),
            db_snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbSnapshotIdentifier"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            iops: pulumi_gestalt_rust::__private::into_domain(o.extract_field("iops")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            license_model: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseModel"),
            ),
            option_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("optionGroupName"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            shared_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sharedAccounts"),
            ),
            snapshot_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotType"),
            ),
            source_db_snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDbSnapshotIdentifier"),
            ),
            source_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceRegion"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            storage_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
