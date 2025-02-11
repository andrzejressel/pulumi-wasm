#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetDiskArgs,
    ) -> GetDiskResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getDisk:getDisk".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDiskResult {
            access_mode: o.get_field("accessMode"),
            async_primary_disks: o.get_field("asyncPrimaryDisks"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            disk_encryption_keys: o.get_field("diskEncryptionKeys"),
            disk_id: o.get_field("diskId"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_confidential_compute: o.get_field("enableConfidentialCompute"),
            guest_os_features: o.get_field("guestOsFeatures"),
            id: o.get_field("id"),
            image: o.get_field("image"),
            interface: o.get_field("interface"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            last_attach_timestamp: o.get_field("lastAttachTimestamp"),
            last_detach_timestamp: o.get_field("lastDetachTimestamp"),
            licenses: o.get_field("licenses"),
            multi_writer: o.get_field("multiWriter"),
            name: o.get_field("name"),
            physical_block_size_bytes: o.get_field("physicalBlockSizeBytes"),
            project: o.get_field("project"),
            provisioned_iops: o.get_field("provisionedIops"),
            provisioned_throughput: o.get_field("provisionedThroughput"),
            pulumi_labels: o.get_field("pulumiLabels"),
            resource_policies: o.get_field("resourcePolicies"),
            self_link: o.get_field("selfLink"),
            size: o.get_field("size"),
            snapshot: o.get_field("snapshot"),
            source_disk: o.get_field("sourceDisk"),
            source_disk_id: o.get_field("sourceDiskId"),
            source_image_encryption_keys: o.get_field("sourceImageEncryptionKeys"),
            source_image_id: o.get_field("sourceImageId"),
            source_snapshot_encryption_keys: o.get_field("sourceSnapshotEncryptionKeys"),
            source_snapshot_id: o.get_field("sourceSnapshotId"),
            storage_pool: o.get_field("storagePool"),
            type_: o.get_field("type"),
            users: o.get_field("users"),
            zone: o.get_field("zone"),
        }
    }
}
