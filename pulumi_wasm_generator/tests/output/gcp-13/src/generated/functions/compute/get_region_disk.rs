pub mod get_region_disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionDiskArgs {
        /// The name of a specific disk.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the region where the disk resides.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionDiskResult {
        pub async_primary_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetRegionDiskAsyncPrimaryDisk>,
        >,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub disk_encryption_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetRegionDiskDiskEncryptionKey>,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub guest_os_features: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetRegionDiskGuestOsFeature>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub interface: pulumi_wasm_rust::Output<String>,
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub last_attach_timestamp: pulumi_wasm_rust::Output<String>,
        pub last_detach_timestamp: pulumi_wasm_rust::Output<String>,
        pub licenses: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub physical_block_size_bytes: pulumi_wasm_rust::Output<i32>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub replica_zones: pulumi_wasm_rust::Output<Vec<String>>,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub size: pulumi_wasm_rust::Output<i32>,
        pub snapshot: pulumi_wasm_rust::Output<String>,
        pub source_disk: pulumi_wasm_rust::Output<String>,
        pub source_disk_id: pulumi_wasm_rust::Output<String>,
        pub source_snapshot_encryption_keys: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionDiskSourceSnapshotEncryptionKey,
            >,
        >,
        pub source_snapshot_id: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub users: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRegionDiskArgs) -> GetRegionDiskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getRegionDisk:getRegionDisk".into(),
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
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "guestOsFeatures".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "physicalBlockSizeBytes".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "replicaZones".into(),
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
                    name: "sourceSnapshotEncryptionKeys".into(),
                },
                register_interface::ResultField {
                    name: "sourceSnapshotId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "users".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRegionDiskResult {
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
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            guest_os_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guestOsFeatures").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            physical_block_size_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("physicalBlockSizeBytes").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            replica_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaZones").unwrap(),
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
            source_snapshot_encryption_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSnapshotEncryptionKeys").unwrap(),
            ),
            source_snapshot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSnapshotId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("users").unwrap(),
            ),
        }
    }
}
