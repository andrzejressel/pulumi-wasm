#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// One or more name/value pairs to filter off of. There are
        /// several valid keys, for a full reference, check out
        /// [describe-snapshots in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ebs::GetSnapshotFilter>>,
        >,
        /// If more than one result is returned, use the most recent snapshot.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Returns the snapshots owned by the specified owner id. Multiple owners can be specified.
        #[builder(into, default)]
        pub owners: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more AWS accounts IDs that can create volumes from the snapshot.
        #[builder(into, default)]
        pub restorable_by_user_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Returns information on a specific snapshot_id.
        #[builder(into, default)]
        pub snapshot_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        /// ARN of the EBS Snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The data encryption key identifier for the snapshot.
        pub data_encryption_key_id: pulumi_gestalt_rust::Output<String>,
        /// Description for the snapshot
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether the snapshot is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ebs::GetSnapshotFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the KMS encryption key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ARN of the Outpost on which the snapshot is stored.
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// Value from an Amazon-maintained list (`amazon`, `aws-marketplace`, `microsoft`) of snapshot owners.
        pub owner_alias: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the EBS snapshot owner.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub owners: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub restorable_by_user_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Snapshot ID (e.g., snap-59fcb34e).
        pub snapshot_id: pulumi_gestalt_rust::Output<String>,
        pub snapshot_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Time stamp when the snapshot was initiated.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// Snapshot state.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Storage tier in which the snapshot is stored.
        pub storage_tier: pulumi_gestalt_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Volume ID (e.g., vol-59fcb34e).
        pub volume_id: pulumi_gestalt_rust::Output<String>,
        /// Size of the drive in GiBs.
        pub volume_size: pulumi_gestalt_rust::Output<i32>,
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
        let filters_binding = args.filters.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let owners_binding = args.owners.get_output(context);
        let restorable_by_user_ids_binding = args
            .restorable_by_user_ids
            .get_output(context);
        let snapshot_ids_binding = args.snapshot_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ebs/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owners".into(),
                    value: &owners_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restorableByUserIds".into(),
                    value: &restorable_by_user_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotIds".into(),
                    value: &snapshot_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSnapshotResult {
            arn: o.get_field("arn"),
            data_encryption_key_id: o.get_field("dataEncryptionKeyId"),
            description: o.get_field("description"),
            encrypted: o.get_field("encrypted"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            most_recent: o.get_field("mostRecent"),
            outpost_arn: o.get_field("outpostArn"),
            owner_alias: o.get_field("ownerAlias"),
            owner_id: o.get_field("ownerId"),
            owners: o.get_field("owners"),
            restorable_by_user_ids: o.get_field("restorableByUserIds"),
            snapshot_id: o.get_field("snapshotId"),
            snapshot_ids: o.get_field("snapshotIds"),
            start_time: o.get_field("startTime"),
            state: o.get_field("state"),
            storage_tier: o.get_field("storageTier"),
            tags: o.get_field("tags"),
            volume_id: o.get_field("volumeId"),
            volume_size: o.get_field("volumeSize"),
        }
    }
}
