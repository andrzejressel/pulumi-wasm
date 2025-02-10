#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// Returns the list of snapshots created by the specific db_instance
        #[builder(into, default)]
        pub db_instance_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific snapshot_id.
        #[builder(into, default)]
        pub db_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set this value to true to include manual DB snapshots that are public and can be
        /// copied or restored by any AWS account, otherwise set this value to false. The default is `false`.
        #[builder(into, default)]
        pub include_public: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Set this value to true to include shared manual DB snapshots from other
        /// AWS accounts that this AWS account has been given permission to copy or restore, otherwise set this value to false.
        /// The default is `false`.
        #[builder(into, default)]
        pub include_shared: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If more than one result is returned, use the most
        /// recent Snapshot.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Type of snapshots to be returned. If you don't specify a SnapshotType
        /// value, then both automated and manual snapshots are returned. Shared and public DB snapshots are not
        /// included in the returned results by default. Possible values are, `automated`, `manual`, `shared`, `public` and `awsbackup`.
        #[builder(into, default)]
        pub snapshot_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the desired DB snapshot.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        /// Allocated storage size in gigabytes (GB).
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Name of the Availability Zone the DB instance was located in at the time of the DB snapshot.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        pub db_instance_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN for the DB snapshot.
        pub db_snapshot_arn: pulumi_gestalt_rust::Output<String>,
        pub db_snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the DB snapshot is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// Name of the database engine.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Version of the database engine.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_public: pulumi_gestalt_rust::Output<Option<bool>>,
        pub include_shared: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Provisioned IOPS (I/O operations per second) value of the DB instance at the time of the snapshot.
        pub iops: pulumi_gestalt_rust::Output<i32>,
        /// ARN for the KMS encryption key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// License model information for the restored DB instance.
        pub license_model: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Provides the option group name for the DB snapshot.
        pub option_group_name: pulumi_gestalt_rust::Output<String>,
        /// Provides the time when the snapshot was taken, in Universal Coordinated Time (UTC). Doesn't change when the snapshot is copied.
        pub original_snapshot_create_time: pulumi_gestalt_rust::Output<String>,
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Provides the time when the snapshot was taken, in Universal Coordinated Time (UTC). Changes for the copy when the snapshot is copied.
        pub snapshot_create_time: pulumi_gestalt_rust::Output<String>,
        pub snapshot_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// DB snapshot ARN that the DB snapshot was copied from. It only has value in case of cross customer or cross region copy.
        pub source_db_snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// Region that the DB snapshot was created in or copied from.
        pub source_region: pulumi_gestalt_rust::Output<String>,
        /// Status of this DB snapshot.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Storage type associated with DB snapshot.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// ID of the VPC associated with the DB snapshot.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSnapshotArgs,
    ) -> GetSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_instance_identifier_binding = args
            .db_instance_identifier
            .get_output(context);
        let db_snapshot_identifier_binding = args
            .db_snapshot_identifier
            .get_output(context);
        let include_public_binding = args.include_public.get_output(context);
        let include_shared_binding = args.include_shared.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let snapshot_type_binding = args.snapshot_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
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
                    name: "includePublic".into(),
                    value: include_public_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeShared".into(),
                    value: include_shared_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: most_recent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotType".into(),
                    value: snapshot_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSnapshotResult {
            allocated_storage: o.get_field("allocatedStorage"),
            availability_zone: o.get_field("availabilityZone"),
            db_instance_identifier: o.get_field("dbInstanceIdentifier"),
            db_snapshot_arn: o.get_field("dbSnapshotArn"),
            db_snapshot_identifier: o.get_field("dbSnapshotIdentifier"),
            encrypted: o.get_field("encrypted"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            id: o.get_field("id"),
            include_public: o.get_field("includePublic"),
            include_shared: o.get_field("includeShared"),
            iops: o.get_field("iops"),
            kms_key_id: o.get_field("kmsKeyId"),
            license_model: o.get_field("licenseModel"),
            most_recent: o.get_field("mostRecent"),
            option_group_name: o.get_field("optionGroupName"),
            original_snapshot_create_time: o.get_field("originalSnapshotCreateTime"),
            port: o.get_field("port"),
            snapshot_create_time: o.get_field("snapshotCreateTime"),
            snapshot_type: o.get_field("snapshotType"),
            source_db_snapshot_identifier: o.get_field("sourceDbSnapshotIdentifier"),
            source_region: o.get_field("sourceRegion"),
            status: o.get_field("status"),
            storage_type: o.get_field("storageType"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
