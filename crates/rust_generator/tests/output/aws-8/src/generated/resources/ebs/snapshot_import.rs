/// Imports a disk image from S3 as a Snapshot.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ebs:SnapshotImport
///     properties:
///       diskContainer:
///         format: VHD
///         userBucket:
///           s3Bucket: disk-images
///           s3Key: source.vhd
///       roleName: disk-image-import
///       tags:
///         Name: HelloWorld
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot_import {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotImportArgs {
        /// The client-specific data. Detailed below.
        #[builder(into, default)]
        pub client_data: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ebs::SnapshotImportClientData>,
        >,
        /// The description string for the import snapshot task.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information about the disk container. Detailed below.
        #[builder(into)]
        pub disk_container: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ebs::SnapshotImportDiskContainer,
        >,
        /// Specifies whether the destination snapshot of the imported image should be encrypted. The default KMS key for EBS is used unless you specify a non-default KMS key using KmsKeyId.
        #[builder(into, default)]
        pub encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An identifier for the symmetric KMS key to use when creating the encrypted snapshot. This parameter is only required if you want to use a non-default KMS key; if this parameter is not specified, the default KMS key for EBS is used. If a KmsKeyId is specified, the Encrypted flag must also be set.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether to permanently restore an archived snapshot.
        #[builder(into, default)]
        pub permanent_restore: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the IAM Role the VM Import/Export service will assume. This role needs certain permissions. See https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-role. Default: `vmimport`
        #[builder(into, default)]
        pub role_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        #[builder(into, default)]
        pub storage_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the snapshot.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        #[builder(into, default)]
        pub temporary_restore_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct SnapshotImportResult {
        /// Amazon Resource Name (ARN) of the EBS Snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The client-specific data. Detailed below.
        pub client_data: pulumi_gestalt_rust::Output<
            Option<super::super::types::ebs::SnapshotImportClientData>,
        >,
        /// The data encryption key identifier for the snapshot.
        pub data_encryption_key_id: pulumi_gestalt_rust::Output<String>,
        /// The description string for the import snapshot task.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Information about the disk container. Detailed below.
        pub disk_container: pulumi_gestalt_rust::Output<
            super::super::types::ebs::SnapshotImportDiskContainer,
        >,
        /// Specifies whether the destination snapshot of the imported image should be encrypted. The default KMS key for EBS is used unless you specify a non-default KMS key using KmsKeyId.
        pub encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An identifier for the symmetric KMS key to use when creating the encrypted snapshot. This parameter is only required if you want to use a non-default KMS key; if this parameter is not specified, the default KMS key for EBS is used. If a KmsKeyId is specified, the Encrypted flag must also be set.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// Value from an Amazon-maintained list (`amazon`, `aws-marketplace`, `microsoft`) of snapshot owners.
        pub owner_alias: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID of the EBS snapshot owner.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to permanently restore an archived snapshot.
        pub permanent_restore: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the IAM Role the VM Import/Export service will assume. This role needs certain permissions. See https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-role. Default: `vmimport`
        pub role_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        pub storage_tier: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the snapshot.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotImportArgs,
    ) -> SnapshotImportResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let client_data_binding = args.client_data.get_output(context);
        let description_binding = args.description.get_output(context);
        let disk_container_binding = args.disk_container.get_output(context);
        let encrypted_binding = args.encrypted.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let permanent_restore_binding = args.permanent_restore.get_output(context);
        let role_name_binding = args.role_name.get_output(context);
        let storage_tier_binding = args.storage_tier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let temporary_restore_days_binding = args
            .temporary_restore_days
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ebs/snapshotImport:SnapshotImport".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientData".into(),
                    value: client_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskContainer".into(),
                    value: disk_container_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encrypted".into(),
                    value: encrypted_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permanentRestore".into(),
                    value: permanent_restore_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleName".into(),
                    value: role_name_binding.get_id(),
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
            ],
        };
        let o = context.register_resource(request);
        SnapshotImportResult {
            arn: o.get_field("arn"),
            client_data: o.get_field("clientData"),
            data_encryption_key_id: o.get_field("dataEncryptionKeyId"),
            description: o.get_field("description"),
            disk_container: o.get_field("diskContainer"),
            encrypted: o.get_field("encrypted"),
            kms_key_id: o.get_field("kmsKeyId"),
            outpost_arn: o.get_field("outpostArn"),
            owner_alias: o.get_field("ownerAlias"),
            owner_id: o.get_field("ownerId"),
            permanent_restore: o.get_field("permanentRestore"),
            role_name: o.get_field("roleName"),
            storage_tier: o.get_field("storageTier"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            temporary_restore_days: o.get_field("temporaryRestoreDays"),
            volume_id: o.get_field("volumeId"),
            volume_size: o.get_field("volumeSize"),
        }
    }
}
