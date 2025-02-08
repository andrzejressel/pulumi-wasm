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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSnapshotArgs,
    ) -> GetSnapshotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let most_recent_binding = args.most_recent.get_output(context).get_inner();
        let owners_binding = args.owners.get_output(context).get_inner();
        let restorable_by_user_ids_binding = args
            .restorable_by_user_ids
            .get_output(context)
            .get_inner();
        let snapshot_ids_binding = args.snapshot_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ebs/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "owners".into(),
                    value: &owners_binding,
                },
                register_interface::ObjectField {
                    name: "restorableByUserIds".into(),
                    value: &restorable_by_user_ids_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotIds".into(),
                    value: &snapshot_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSnapshotResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            data_encryption_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataEncryptionKeyId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            most_recent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mostRecent"),
            ),
            outpost_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            owner_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerAlias"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            owners: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("owners"),
            ),
            restorable_by_user_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restorableByUserIds"),
            ),
            snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotId"),
            ),
            snapshot_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotIds"),
            ),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            storage_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageTier"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            volume_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeId"),
            ),
            volume_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeSize"),
            ),
        }
    }
}
