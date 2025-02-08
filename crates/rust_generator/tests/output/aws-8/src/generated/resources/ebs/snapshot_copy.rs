/// Creates a Snapshot of a snapshot.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ebs:Volume
///     properties:
///       availabilityZone: us-west-2a
///       size: 40
///       tags:
///         Name: HelloWorld
///   exampleSnapshot:
///     type: aws:ebs:Snapshot
///     name: example_snapshot
///     properties:
///       volumeId: ${example.id}
///       tags:
///         Name: HelloWorld_snap
///   exampleCopy:
///     type: aws:ebs:SnapshotCopy
///     name: example_copy
///     properties:
///       sourceSnapshotId: ${exampleSnapshot.id}
///       sourceRegion: us-west-2
///       tags:
///         Name: HelloWorld_copy_snap
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot_copy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotCopyArgs {
        /// Specifies a completion duration to initiate a time-based snapshot copy. Time-based snapshot copy operations complete within the specified duration.  Value must be between 15 and 2880 minutes, in 15 minute increments only.
        #[builder(into, default)]
        pub completion_duration_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A description of what the snapshot is.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the snapshot is encrypted.
        #[builder(into, default)]
        pub encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ARN for the KMS encryption key.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether to permanently restore an archived snapshot.
        #[builder(into, default)]
        pub permanent_restore: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The region of the source snapshot.
        #[builder(into)]
        pub source_region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN for the snapshot to be copied.
        #[builder(into)]
        pub source_snapshot_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        #[builder(into, default)]
        pub storage_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags for the snapshot.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        #[builder(into, default)]
        pub temporary_restore_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct SnapshotCopyResult {
        /// Amazon Resource Name (ARN) of the EBS Snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies a completion duration to initiate a time-based snapshot copy. Time-based snapshot copy operations complete within the specified duration.  Value must be between 15 and 2880 minutes, in 15 minute increments only.
        pub completion_duration_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The data encryption key identifier for the snapshot.
        pub data_encryption_key_id: pulumi_gestalt_rust::Output<String>,
        /// A description of what the snapshot is.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the snapshot is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN for the KMS encryption key.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// Value from an Amazon-maintained list (`amazon`, `aws-marketplace`, `microsoft`) of snapshot owners.
        pub owner_alias: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID of the snapshot owner.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to permanently restore an archived snapshot.
        pub permanent_restore: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The region of the source snapshot.
        pub source_region: pulumi_gestalt_rust::Output<String>,
        /// The ARN for the snapshot to be copied.
        pub source_snapshot_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        pub storage_tier: pulumi_gestalt_rust::Output<String>,
        /// A map of tags for the snapshot.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        pub temporary_restore_days: pulumi_gestalt_rust::Output<Option<i32>>,
        pub volume_id: pulumi_gestalt_rust::Output<String>,
        /// The size of the drive in GiBs.
        pub volume_size: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SnapshotCopyArgs,
    ) -> SnapshotCopyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let completion_duration_minutes_binding = args
            .completion_duration_minutes
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let encrypted_binding = args.encrypted.get_output(context).get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let permanent_restore_binding = args
            .permanent_restore
            .get_output(context)
            .get_inner();
        let source_region_binding = args.source_region.get_output(context).get_inner();
        let source_snapshot_id_binding = args
            .source_snapshot_id
            .get_output(context)
            .get_inner();
        let storage_tier_binding = args.storage_tier.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let temporary_restore_days_binding = args
            .temporary_restore_days
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ebs/snapshotCopy:SnapshotCopy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "completionDurationMinutes".into(),
                    value: &completion_duration_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encrypted".into(),
                    value: &encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "permanentRestore".into(),
                    value: &permanent_restore_binding,
                },
                register_interface::ObjectField {
                    name: "sourceRegion".into(),
                    value: &source_region_binding,
                },
                register_interface::ObjectField {
                    name: "sourceSnapshotId".into(),
                    value: &source_snapshot_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageTier".into(),
                    value: &storage_tier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "temporaryRestoreDays".into(),
                    value: &temporary_restore_days_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SnapshotCopyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            completion_duration_minutes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("completionDurationMinutes"),
            ),
            data_encryption_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataEncryptionKeyId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
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
            permanent_restore: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permanentRestore"),
            ),
            source_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceRegion"),
            ),
            source_snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSnapshotId"),
            ),
            storage_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageTier"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            temporary_restore_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("temporaryRestoreDays"),
            ),
            volume_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeId"),
            ),
            volume_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeSize"),
            ),
        }
    }
}
