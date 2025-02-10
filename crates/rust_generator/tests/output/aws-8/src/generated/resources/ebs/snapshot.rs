/// Creates a Snapshot of an EBS Volume.
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
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EBS Snapshot using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/snapshot:Snapshot id snap-049df61146c4d7901
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// A description of what the snapshot is.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.
        #[builder(into, default)]
        pub outpost_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether to permanently restore an archived snapshot.
        #[builder(into, default)]
        pub permanent_restore: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        #[builder(into, default)]
        pub storage_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the snapshot. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        #[builder(into, default)]
        pub temporary_restore_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Volume ID of which to make a snapshot.
        #[builder(into)]
        pub volume_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// Amazon Resource Name (ARN) of the EBS Snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The data encryption key identifier for the snapshot.
        pub data_encryption_key_id: pulumi_gestalt_rust::Output<String>,
        /// A description of what the snapshot is.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the snapshot is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// The ARN for the KMS encryption key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.
        pub outpost_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Value from an Amazon-maintained list (`amazon`, `aws-marketplace`, `microsoft`) of snapshot owners.
        pub owner_alias: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID of the EBS snapshot owner.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to permanently restore an archived snapshot.
        pub permanent_restore: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        pub storage_tier: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the snapshot. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        pub temporary_restore_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Volume ID of which to make a snapshot.
        pub volume_id: pulumi_gestalt_rust::Output<String>,
        /// The size of the drive in GiBs.
        pub volume_size: pulumi_gestalt_rust::Output<i32>,
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
        let description_binding = args.description.get_output(context);
        let outpost_arn_binding = args.outpost_arn.get_output(context);
        let permanent_restore_binding = args.permanent_restore.get_output(context);
        let storage_tier_binding = args.storage_tier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let temporary_restore_days_binding = args
            .temporary_restore_days
            .get_output(context);
        let volume_id_binding = args.volume_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ebs/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outpostArn".into(),
                    value: outpost_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permanentRestore".into(),
                    value: permanent_restore_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageTier".into(),
                    value: storage_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "temporaryRestoreDays".into(),
                    value: temporary_restore_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeId".into(),
                    value: volume_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotResult {
            arn: o.get_field("arn"),
            data_encryption_key_id: o.get_field("dataEncryptionKeyId"),
            description: o.get_field("description"),
            encrypted: o.get_field("encrypted"),
            kms_key_id: o.get_field("kmsKeyId"),
            outpost_arn: o.get_field("outpostArn"),
            owner_alias: o.get_field("ownerAlias"),
            owner_id: o.get_field("ownerId"),
            permanent_restore: o.get_field("permanentRestore"),
            storage_tier: o.get_field("storageTier"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            temporary_restore_days: o.get_field("temporaryRestoreDays"),
            volume_id: o.get_field("volumeId"),
            volume_size: o.get_field("volumeSize"),
        }
    }
}
