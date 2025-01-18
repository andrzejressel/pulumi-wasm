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
pub mod snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// A description of what the snapshot is.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.
        #[builder(into, default)]
        pub outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether to permanently restore an archived snapshot.
        #[builder(into, default)]
        pub permanent_restore: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        #[builder(into, default)]
        pub storage_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the snapshot. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        #[builder(into, default)]
        pub temporary_restore_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Volume ID of which to make a snapshot.
        #[builder(into)]
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// Amazon Resource Name (ARN) of the EBS Snapshot.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The data encryption key identifier for the snapshot.
        pub data_encryption_key_id: pulumi_wasm_rust::Output<String>,
        /// A description of what the snapshot is.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the snapshot is encrypted.
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        /// The ARN for the KMS encryption key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.
        pub outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Value from an Amazon-maintained list (`amazon`, `aws-marketplace`, `microsoft`) of snapshot owners.
        pub owner_alias: pulumi_wasm_rust::Output<String>,
        /// The AWS account ID of the EBS snapshot owner.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether to permanently restore an archived snapshot.
        pub permanent_restore: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the storage tier. Valid values are `archive` and `standard`. Default value is `standard`.
        pub storage_tier: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the snapshot. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.
        pub temporary_restore_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Volume ID of which to make a snapshot.
        pub volume_id: pulumi_wasm_rust::Output<String>,
        /// The size of the drive in GiBs.
        pub volume_size: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SnapshotArgs) -> SnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let outpost_arn_binding = args.outpost_arn.get_inner();
        let permanent_restore_binding = args.permanent_restore.get_inner();
        let storage_tier_binding = args.storage_tier.get_inner();
        let tags_binding = args.tags.get_inner();
        let temporary_restore_days_binding = args.temporary_restore_days.get_inner();
        let volume_id_binding = args.volume_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ebs/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "outpostArn".into(),
                    value: &outpost_arn_binding,
                },
                register_interface::ObjectField {
                    name: "permanentRestore".into(),
                    value: &permanent_restore_binding,
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
                register_interface::ObjectField {
                    name: "volumeId".into(),
                    value: &volume_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dataEncryptionKeyId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
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
        SnapshotResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            data_encryption_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataEncryptionKeyId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
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
