#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_region_disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionDiskArgs {
        /// The name of a specific disk.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the region where the disk resides.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionDiskResult {
        pub async_primary_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRegionDiskAsyncPrimaryDisk>,
        >,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub disk_encryption_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRegionDiskDiskEncryptionKey>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub guest_os_features: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRegionDiskGuestOsFeature>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub interface: pulumi_gestalt_rust::Output<String>,
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub last_attach_timestamp: pulumi_gestalt_rust::Output<String>,
        pub last_detach_timestamp: pulumi_gestalt_rust::Output<String>,
        pub licenses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub physical_block_size_bytes: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub replica_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub size: pulumi_gestalt_rust::Output<i32>,
        pub snapshot: pulumi_gestalt_rust::Output<String>,
        pub source_disk: pulumi_gestalt_rust::Output<String>,
        pub source_disk_id: pulumi_gestalt_rust::Output<String>,
        pub source_snapshot_encryption_keys: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionDiskSourceSnapshotEncryptionKey,
            >,
        >,
        pub source_snapshot_id: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub users: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegionDiskArgs,
    ) -> GetRegionDiskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getRegionDisk:getRegionDisk".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegionDiskResult {
            async_primary_disks: o.get_field("asyncPrimaryDisks"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            disk_encryption_keys: o.get_field("diskEncryptionKeys"),
            effective_labels: o.get_field("effectiveLabels"),
            guest_os_features: o.get_field("guestOsFeatures"),
            id: o.get_field("id"),
            interface: o.get_field("interface"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            last_attach_timestamp: o.get_field("lastAttachTimestamp"),
            last_detach_timestamp: o.get_field("lastDetachTimestamp"),
            licenses: o.get_field("licenses"),
            name: o.get_field("name"),
            physical_block_size_bytes: o.get_field("physicalBlockSizeBytes"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            replica_zones: o.get_field("replicaZones"),
            self_link: o.get_field("selfLink"),
            size: o.get_field("size"),
            snapshot: o.get_field("snapshot"),
            source_disk: o.get_field("sourceDisk"),
            source_disk_id: o.get_field("sourceDiskId"),
            source_snapshot_encryption_keys: o.get_field("sourceSnapshotEncryptionKeys"),
            source_snapshot_id: o.get_field("sourceSnapshotId"),
            type_: o.get_field("type"),
            users: o.get_field("users"),
        }
    }
}
