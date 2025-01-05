pub mod get_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// One or more name/value pairs to filter off of. There are
        /// several valid keys, for a full reference, check out
        /// [describe-snapshots in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ebs::GetSnapshotFilter>>,
        >,
        /// If more than one result is returned, use the most recent snapshot.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Returns the snapshots owned by the specified owner id. Multiple owners can be specified.
        #[builder(into, default)]
        pub owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more AWS accounts IDs that can create volumes from the snapshot.
        #[builder(into, default)]
        pub restorable_by_user_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Returns information on a specific snapshot_id.
        #[builder(into, default)]
        pub snapshot_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        /// ARN of the EBS Snapshot.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The data encryption key identifier for the snapshot.
        pub data_encryption_key_id: pulumi_wasm_rust::Output<String>,
        /// Description for the snapshot
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether the snapshot is encrypted.
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ebs::GetSnapshotFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN for the KMS encryption key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// ARN of the Outpost on which the snapshot is stored.
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// Value from an Amazon-maintained list (`amazon`, `aws-marketplace`, `microsoft`) of snapshot owners.
        pub owner_alias: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the EBS snapshot owner.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub restorable_by_user_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Snapshot ID (e.g., snap-59fcb34e).
        pub snapshot_id: pulumi_wasm_rust::Output<String>,
        pub snapshot_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Time stamp when the snapshot was initiated.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// Snapshot state.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Storage tier in which the snapshot is stored.
        pub storage_tier: pulumi_wasm_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Volume ID (e.g., vol-59fcb34e).
        pub volume_id: pulumi_wasm_rust::Output<String>,
        /// Size of the drive in GiBs.
        pub volume_size: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSnapshotArgs) -> GetSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let most_recent_binding = args.most_recent.get_inner();
        let owners_binding = args.owners.get_inner();
        let restorable_by_user_ids_binding = args.restorable_by_user_ids.get_inner();
        let snapshot_ids_binding = args.snapshot_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ebs/getSnapshot:getSnapshot".into(),
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
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "mostRecent".into(),
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
                    name: "owners".into(),
                },
                register_interface::ResultField {
                    name: "restorableByUserIds".into(),
                },
                register_interface::ResultField {
                    name: "snapshotId".into(),
                },
                register_interface::ResultField {
                    name: "snapshotIds".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "storageTier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "volumeId".into(),
                },
                register_interface::ResultField {
                    name: "volumeSize".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSnapshotResult {
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
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecent").unwrap(),
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
            owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owners").unwrap(),
            ),
            restorable_by_user_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restorableByUserIds").unwrap(),
            ),
            snapshot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotId").unwrap(),
            ),
            snapshot_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotIds").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            storage_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageTier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
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
