/// Manages an RDS database instance snapshot copy. For managing RDS database cluster snapshots, see the `aws.rds.ClusterSnapshot` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot_copy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotCopyArgs {
        /// Whether to copy existing tags. Defaults to `false`.
        #[builder(into, default)]
        pub copy_tags: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Destination region to place snapshot copy.
        #[builder(into, default)]
        pub destination_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// KMS key ID.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of an option group to associate with the copy of the snapshot.
        #[builder(into, default)]
        pub option_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// he URL that contains a Signature Version 4 signed request.
        #[builder(into, default)]
        pub presigned_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        #[builder(into, default)]
        pub shared_accounts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Snapshot identifier of the source snapshot.
        #[builder(into)]
        pub source_db_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The external custom Availability Zone.
        #[builder(into, default)]
        pub target_custom_availability_zone: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Identifier for the snapshot.
        #[builder(into)]
        pub target_db_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotCopyResult {
        /// Specifies the allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the Availability Zone the DB instance was located in at the time of the DB snapshot.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Whether to copy existing tags. Defaults to `false`.
        pub copy_tags: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) for the DB snapshot.
        pub db_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        /// The Destination region to place snapshot copy.
        pub destination_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the DB snapshot is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the name of the database engine.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Specifies the version of the database engine.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.
        pub iops: pulumi_gestalt_rust::Output<i32>,
        /// KMS key ID.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// License model information for the restored DB instance.
        pub license_model: pulumi_gestalt_rust::Output<String>,
        /// The name of an option group to associate with the copy of the snapshot.
        pub option_group_name: pulumi_gestalt_rust::Output<String>,
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// he URL that contains a Signature Version 4 signed request.
        pub presigned_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of AWS Account IDs to share the snapshot with. Use `all` to make the snapshot public.
        pub shared_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub snapshot_type: pulumi_gestalt_rust::Output<String>,
        /// Snapshot identifier of the source snapshot.
        pub source_db_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// The region that the DB snapshot was created in or copied from.
        pub source_region: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage type associated with DB snapshot.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The external custom Availability Zone.
        pub target_custom_availability_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Identifier for the snapshot.
        pub target_db_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
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
        args: SnapshotCopyArgs,
    ) -> SnapshotCopyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let copy_tags_binding = args.copy_tags.get_output(context);
        let destination_region_binding = args.destination_region.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let option_group_name_binding = args.option_group_name.get_output(context);
        let presigned_url_binding = args.presigned_url.get_output(context);
        let shared_accounts_binding = args.shared_accounts.get_output(context);
        let source_db_snapshot_identifier_binding = args
            .source_db_snapshot_identifier
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_custom_availability_zone_binding = args
            .target_custom_availability_zone
            .get_output(context);
        let target_db_snapshot_identifier_binding = args
            .target_db_snapshot_identifier
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/snapshotCopy:SnapshotCopy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyTags".into(),
                    value: &copy_tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationRegion".into(),
                    value: &destination_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optionGroupName".into(),
                    value: &option_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "presignedUrl".into(),
                    value: &presigned_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedAccounts".into(),
                    value: &shared_accounts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceDbSnapshotIdentifier".into(),
                    value: &source_db_snapshot_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetCustomAvailabilityZone".into(),
                    value: &target_custom_availability_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetDbSnapshotIdentifier".into(),
                    value: &target_db_snapshot_identifier_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotCopyResult {
            allocated_storage: o.get_field("allocatedStorage"),
            availability_zone: o.get_field("availabilityZone"),
            copy_tags: o.get_field("copyTags"),
            db_snapshot_arn: o.get_field("dbSnapshotArn"),
            destination_region: o.get_field("destinationRegion"),
            encrypted: o.get_field("encrypted"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            iops: o.get_field("iops"),
            kms_key_id: o.get_field("kmsKeyId"),
            license_model: o.get_field("licenseModel"),
            option_group_name: o.get_field("optionGroupName"),
            port: o.get_field("port"),
            presigned_url: o.get_field("presignedUrl"),
            shared_accounts: o.get_field("sharedAccounts"),
            snapshot_type: o.get_field("snapshotType"),
            source_db_snapshot_identifier: o.get_field("sourceDbSnapshotIdentifier"),
            source_region: o.get_field("sourceRegion"),
            storage_type: o.get_field("storageType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_custom_availability_zone: o.get_field("targetCustomAvailabilityZone"),
            target_db_snapshot_identifier: o.get_field("targetDbSnapshotIdentifier"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
