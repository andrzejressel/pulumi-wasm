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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotArgs,
    ) -> SnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_instance_identifier_binding = args
            .db_instance_identifier
            .get_output(context);
        let db_snapshot_identifier_binding = args
            .db_snapshot_identifier
            .get_output(context);
        let shared_accounts_binding = args.shared_accounts.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbInstanceIdentifier".into(),
                    value: db_instance_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbSnapshotIdentifier".into(),
                    value: db_snapshot_identifier_binding.get_id(),
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
        SnapshotResult {
            allocated_storage: o.get_field("allocatedStorage"),
            availability_zone: o.get_field("availabilityZone"),
            db_instance_identifier: o.get_field("dbInstanceIdentifier"),
            db_snapshot_arn: o.get_field("dbSnapshotArn"),
            db_snapshot_identifier: o.get_field("dbSnapshotIdentifier"),
            encrypted: o.get_field("encrypted"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            iops: o.get_field("iops"),
            kms_key_id: o.get_field("kmsKeyId"),
            license_model: o.get_field("licenseModel"),
            option_group_name: o.get_field("optionGroupName"),
            port: o.get_field("port"),
            shared_accounts: o.get_field("sharedAccounts"),
            snapshot_type: o.get_field("snapshotType"),
            source_db_snapshot_identifier: o.get_field("sourceDbSnapshotIdentifier"),
            source_region: o.get_field("sourceRegion"),
            status: o.get_field("status"),
            storage_type: o.get_field("storageType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
