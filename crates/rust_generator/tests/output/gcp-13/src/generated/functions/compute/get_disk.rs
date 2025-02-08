#[allow(clippy::doc_lazy_continuation)]
pub mod get_disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDiskArgs {
        /// The name of a specific disk.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the zone where the disk resides.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDiskResult {
        pub access_mode: pulumi_gestalt_rust::Output<String>,
        pub async_primary_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetDiskAsyncPrimaryDisk>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub disk_encryption_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetDiskDiskEncryptionKey>,
        >,
        pub disk_id: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub enable_confidential_compute: pulumi_gestalt_rust::Output<bool>,
        pub guest_os_features: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetDiskGuestOsFeature>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The image from which to initialize this disk.
        pub image: pulumi_gestalt_rust::Output<String>,
        pub interface: pulumi_gestalt_rust::Output<String>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Last attach timestamp in RFC3339 text format.
        pub last_attach_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Last detach timestamp in RFC3339 text format.
        pub last_detach_timestamp: pulumi_gestalt_rust::Output<String>,
        pub licenses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub multi_writer: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Physical block size of the persistent disk, in bytes.
        pub physical_block_size_bytes: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub provisioned_iops: pulumi_gestalt_rust::Output<i32>,
        pub provisioned_throughput: pulumi_gestalt_rust::Output<i32>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_policies: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Size of the persistent disk, specified in GB.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// The source snapshot used to create this disk.
        pub snapshot: pulumi_gestalt_rust::Output<String>,
        pub source_disk: pulumi_gestalt_rust::Output<String>,
        pub source_disk_id: pulumi_gestalt_rust::Output<String>,
        /// The customer-supplied encryption key of the source image.
        pub source_image_encryption_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetDiskSourceImageEncryptionKey>,
        >,
        /// The ID value of the image used to create this disk. This value
        /// identifies the exact image that was used to create this persistent
        /// disk. For example, if you created the persistent disk from an image
        /// that was later deleted and recreated under the same name, the source
        /// image ID would identify the exact version of the image that was used.
        pub source_image_id: pulumi_gestalt_rust::Output<String>,
        /// The customer-supplied encryption key of the source snapshot.
        pub source_snapshot_encryption_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetDiskSourceSnapshotEncryptionKey>,
        >,
        /// The unique ID of the snapshot used to create this disk. This value
        /// identifies the exact snapshot that was used to create this persistent
        /// disk. For example, if you created the persistent disk from a snapshot
        /// that was later deleted and recreated under the same name, the source
        /// snapshot ID would identify the exact version of the snapshot that was
        /// used.
        pub source_snapshot_id: pulumi_gestalt_rust::Output<String>,
        pub storage_pool: pulumi_gestalt_rust::Output<String>,
        /// URL of the disk type resource describing which disk type to use to
        /// create the disk.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Links to the users of the disk (attached instances) in form:
        /// project/zones/zone/instances/instance
        pub users: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A reference to the zone where the disk resides.
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDiskArgs,
    ) -> GetDiskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDiskResult {
            access_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessMode"),
            ),
            async_primary_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("asyncPrimaryDisks"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disk_encryption_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskEncryptionKeys"),
            ),
            disk_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskId"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            enable_confidential_compute: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableConfidentialCompute"),
            ),
            guest_os_features: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("guestOsFeatures"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            image: pulumi_gestalt_rust::__private::into_domain(o.extract_field("image")),
            interface: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interface"),
            ),
            label_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            last_attach_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastAttachTimestamp"),
            ),
            last_detach_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastDetachTimestamp"),
            ),
            licenses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenses"),
            ),
            multi_writer: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiWriter"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            physical_block_size_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("physicalBlockSizeBytes"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            provisioned_iops: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisionedIops"),
            ),
            provisioned_throughput: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisionedThroughput"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            resource_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourcePolicies"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            size: pulumi_gestalt_rust::__private::into_domain(o.extract_field("size")),
            snapshot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshot"),
            ),
            source_disk: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDisk"),
            ),
            source_disk_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDiskId"),
            ),
            source_image_encryption_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceImageEncryptionKeys"),
            ),
            source_image_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceImageId"),
            ),
            source_snapshot_encryption_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSnapshotEncryptionKeys"),
            ),
            source_snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSnapshotId"),
            ),
            storage_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storagePool"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            users: pulumi_gestalt_rust::__private::into_domain(o.extract_field("users")),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
