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
pub mod snapshot_import {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotImportArgs {
        /// The client-specific data. Detailed below.
        #[builder(into, default)]
        pub client_data: pulumi_wasm_rust::Output<
            Option<super::super::types::ebs::SnapshotImportClientData>,
        >,
        /// The description string for the import snapshot task.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Information about the disk container. Detailed below.
        #[builder(into)]
        pub disk_container: pulumi_wasm_rust::Output<
            super::super::types::ebs::SnapshotImportDiskContainer,
        >,
        /// Specifies whether the destination snapshot of the imported image should be encrypted. The default KMS key for EBS is used unless you specify a non-default KMS key using KmsKeyId.
        #[builder(into, default)]
        pub encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// An identifier for the symmetric KMS key to use when creating the encrypted snapshot. This parameter is only required if you want to use a non-default KMS key; if this parameter is not specified, the default KMS key for EBS is used. If a KmsKeyId is specified, the Encrypted flag must also be set.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether to permanently restore an archived snapshot.
        #[builder(into, default)]
        pub permanent_restore: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the IAM Role the VM Import/Export service will assume. This role needs certain permissions. See https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-role. Default: `vmimport`
        #[builder(into, default)]
        pub role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        #[builder(into, default)]
        pub storage_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the snapshot.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        #[builder(into, default)]
        pub temporary_restore_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct SnapshotImportResult {
        /// Amazon Resource Name (ARN) of the EBS Snapshot.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The client-specific data. Detailed below.
        pub client_data: pulumi_wasm_rust::Output<
            Option<super::super::types::ebs::SnapshotImportClientData>,
        >,
        /// The data encryption key identifier for the snapshot.
        pub data_encryption_key_id: pulumi_wasm_rust::Output<String>,
        /// The description string for the import snapshot task.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Information about the disk container. Detailed below.
        pub disk_container: pulumi_wasm_rust::Output<
            super::super::types::ebs::SnapshotImportDiskContainer,
        >,
        /// Specifies whether the destination snapshot of the imported image should be encrypted. The default KMS key for EBS is used unless you specify a non-default KMS key using KmsKeyId.
        pub encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// An identifier for the symmetric KMS key to use when creating the encrypted snapshot. This parameter is only required if you want to use a non-default KMS key; if this parameter is not specified, the default KMS key for EBS is used. If a KmsKeyId is specified, the Encrypted flag must also be set.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// Value from an Amazon-maintained list (`amazon`, `aws-marketplace`, `microsoft`) of snapshot owners.
        pub owner_alias: pulumi_wasm_rust::Output<String>,
        /// The AWS account ID of the EBS snapshot owner.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether to permanently restore an archived snapshot.
        pub permanent_restore: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the IAM Role the VM Import/Export service will assume. This role needs certain permissions. See https://docs.aws.amazon.com/vm-import/latest/userguide/vmie_prereqs.html#vmimport-role. Default: `vmimport`
        pub role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        pub storage_tier: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the snapshot.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        pub temporary_restore_days: pulumi_wasm_rust::Output<Option<i32>>,
        pub volume_id: pulumi_wasm_rust::Output<String>,
        /// The size of the drive in GiBs.
        pub volume_size: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SnapshotImportArgs) -> SnapshotImportResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_data_binding = args.client_data.get_inner();
        let description_binding = args.description.get_inner();
        let disk_container_binding = args.disk_container.get_inner();
        let encrypted_binding = args.encrypted.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let permanent_restore_binding = args.permanent_restore.get_inner();
        let role_name_binding = args.role_name.get_inner();
        let storage_tier_binding = args.storage_tier.get_inner();
        let tags_binding = args.tags.get_inner();
        let temporary_restore_days_binding = args.temporary_restore_days.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ebs/snapshotImport:SnapshotImport".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientData".into(),
                    value: &client_data_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "diskContainer".into(),
                    value: &disk_container_binding,
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
                    name: "roleName".into(),
                    value: &role_name_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clientData".into(),
                },
                register_interface::ResultField {
                    name: "dataEncryptionKeyId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "diskContainer".into(),
                },
                register_interface::ResultField {
                    name: "encrypted".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "outpostArn".into(),
                },
                register_interface::ResultField {
                    name: "ownerAlias".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "permanentRestore".into(),
                },
                register_interface::ResultField {
                    name: "roleName".into(),
                },
                register_interface::ResultField {
                    name: "storageTier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "temporaryRestoreDays".into(),
                },
                register_interface::ResultField {
                    name: "volumeId".into(),
                },
                register_interface::ResultField {
                    name: "volumeSize".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SnapshotImportResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            client_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientData").unwrap(),
            ),
            data_encryption_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataEncryptionKeyId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disk_container: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskContainer").unwrap(),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encrypted").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostArn").unwrap(),
            ),
            owner_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAlias").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            permanent_restore: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permanentRestore").unwrap(),
            ),
            role_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleName").unwrap(),
            ),
            storage_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageTier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            temporary_restore_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("temporaryRestoreDays").unwrap(),
            ),
            volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeId").unwrap(),
            ),
            volume_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeSize").unwrap(),
            ),
        }
    }
}
