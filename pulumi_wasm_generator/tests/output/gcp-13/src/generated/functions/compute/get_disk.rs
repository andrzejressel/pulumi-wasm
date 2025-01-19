pub mod get_disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDiskArgs {
        /// The name of a specific disk.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the zone where the disk resides.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDiskResult {
        pub access_mode: pulumi_wasm_rust::Output<String>,
        pub async_primary_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetDiskAsyncPrimaryDisk>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// The optional description of this resource.
        pub description: pulumi_wasm_rust::Output<String>,
        pub disk_encryption_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetDiskDiskEncryptionKey>,
        >,
        pub disk_id: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub enable_confidential_compute: pulumi_wasm_rust::Output<bool>,
        pub guest_os_features: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetDiskGuestOsFeature>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The image from which to initialize this disk.
        pub image: pulumi_wasm_rust::Output<String>,
        pub interface: pulumi_wasm_rust::Output<String>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Last attach timestamp in RFC3339 text format.
        pub last_attach_timestamp: pulumi_wasm_rust::Output<String>,
        /// Last detach timestamp in RFC3339 text format.
        pub last_detach_timestamp: pulumi_wasm_rust::Output<String>,
        pub licenses: pulumi_wasm_rust::Output<Vec<String>>,
        pub multi_writer: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Physical block size of the persistent disk, in bytes.
        pub physical_block_size_bytes: pulumi_wasm_rust::Output<i32>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub provisioned_iops: pulumi_wasm_rust::Output<i32>,
        pub provisioned_throughput: pulumi_wasm_rust::Output<i32>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_policies: pulumi_wasm_rust::Output<Vec<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Size of the persistent disk, specified in GB.
        pub size: pulumi_wasm_rust::Output<i32>,
        /// The source snapshot used to create this disk.
        pub snapshot: pulumi_wasm_rust::Output<String>,
        pub source_disk: pulumi_wasm_rust::Output<String>,
        pub source_disk_id: pulumi_wasm_rust::Output<String>,
        /// The customer-supplied encryption key of the source image.
        pub source_image_encryption_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetDiskSourceImageEncryptionKey>,
        >,
        /// The ID value of the image used to create this disk. This value
        /// identifies the exact image that was used to create this persistent
        /// disk. For example, if you created the persistent disk from an image
        /// that was later deleted and recreated under the same name, the source
        /// image ID would identify the exact version of the image that was used.
        pub source_image_id: pulumi_wasm_rust::Output<String>,
        /// The customer-supplied encryption key of the source snapshot.
        pub source_snapshot_encryption_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetDiskSourceSnapshotEncryptionKey>,
        >,
        /// The unique ID of the snapshot used to create this disk. This value
        /// identifies the exact snapshot that was used to create this persistent
        /// disk. For example, if you created the persistent disk from a snapshot
        /// that was later deleted and recreated under the same name, the source
        /// snapshot ID would identify the exact version of the snapshot that was
        /// used.
        pub source_snapshot_id: pulumi_wasm_rust::Output<String>,
        pub storage_pool: pulumi_wasm_rust::Output<String>,
        /// URL of the disk type resource describing which disk type to use to
        /// create the disk.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Links to the users of the disk (attached instances) in form:
        /// project/zones/zone/instances/instance
        pub users: pulumi_wasm_rust::Output<Vec<String>>,
        /// A reference to the zone where the disk resides.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDiskArgs) -> GetDiskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getDisk:getDisk".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessMode".into(),
                },
                register_interface::ResultField {
                    name: "asyncPrimaryDisks".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "diskEncryptionKeys".into(),
                },
                register_interface::ResultField {
                    name: "diskId".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "enableConfidentialCompute".into(),
                },
                register_interface::ResultField {
                    name: "guestOsFeatures".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "image".into(),
                },
                register_interface::ResultField {
                    name: "interface".into(),
                },
                register_interface::ResultField {
                    name: "labelFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lastAttachTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "lastDetachTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "licenses".into(),
                },
                register_interface::ResultField {
                    name: "multiWriter".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "physicalBlockSizeBytes".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "provisionedIops".into(),
                },
                register_interface::ResultField {
                    name: "provisionedThroughput".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "resourcePolicies".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "size".into(),
                },
                register_interface::ResultField {
                    name: "snapshot".into(),
                },
                register_interface::ResultField {
                    name: "sourceDisk".into(),
                },
                register_interface::ResultField {
                    name: "sourceDiskId".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageEncryptionKeys".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageId".into(),
                },
                register_interface::ResultField {
                    name: "sourceSnapshotEncryptionKeys".into(),
                },
                register_interface::ResultField {
                    name: "sourceSnapshotId".into(),
                },
                register_interface::ResultField {
                    name: "storagePool".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "users".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDiskResult {
            access_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessMode").unwrap(),
            ),
            async_primary_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("asyncPrimaryDisks").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disk_encryption_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskEncryptionKeys").unwrap(),
            ),
            disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskId").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            enable_confidential_compute: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableConfidentialCompute").unwrap(),
            ),
            guest_os_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guestOsFeatures").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("image").unwrap(),
            ),
            interface: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interface").unwrap(),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelFingerprint").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            last_attach_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastAttachTimestamp").unwrap(),
            ),
            last_detach_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastDetachTimestamp").unwrap(),
            ),
            licenses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenses").unwrap(),
            ),
            multi_writer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiWriter").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            physical_block_size_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("physicalBlockSizeBytes").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            provisioned_iops: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisionedIops").unwrap(),
            ),
            provisioned_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisionedThroughput").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            resource_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourcePolicies").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("size").unwrap(),
            ),
            snapshot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshot").unwrap(),
            ),
            source_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDisk").unwrap(),
            ),
            source_disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDiskId").unwrap(),
            ),
            source_image_encryption_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageEncryptionKeys").unwrap(),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageId").unwrap(),
            ),
            source_snapshot_encryption_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSnapshotEncryptionKeys").unwrap(),
            ),
            source_snapshot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSnapshotId").unwrap(),
            ),
            storage_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storagePool").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("users").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
