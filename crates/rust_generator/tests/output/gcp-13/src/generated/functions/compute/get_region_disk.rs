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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRegionDiskArgs,
    ) -> GetRegionDiskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegionDiskResult {
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
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            guest_os_features: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("guestOsFeatures"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            physical_block_size_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("physicalBlockSizeBytes"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            replica_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicaZones"),
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
            source_snapshot_encryption_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSnapshotEncryptionKeys"),
            ),
            source_snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSnapshotId"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            users: pulumi_gestalt_rust::__private::into_domain(o.extract_field("users")),
        }
    }
}
