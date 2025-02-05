/// Persistent disks are durable storage devices that function similarly to
/// the physical disks in a desktop or a server. Compute Engine manages the
/// hardware behind these devices to ensure data redundancy and optimize
/// performance for you. Persistent disks are available as either standard
/// hard disk drives (HDD) or solid-state drives (SSD).
///
/// Persistent disks are located independently from your virtual machine
/// instances, so you can detach or move persistent disks to keep your data
/// even after you delete your instances. Persistent disk performance scales
/// automatically with size, so you can resize your existing persistent disks
/// or add more persistent disks to an instance to meet your performance and
/// storage space requirements.
///
/// Add a persistent disk to your instance when you need reliable and
/// affordable storage with consistent performance characteristics.
///
///
/// To get more information about RegionDisk, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionDisks)
/// * How-to Guides
///     * [Adding or Resizing Regional Persistent Disks](https://cloud.google.com/compute/docs/disks/regional-persistent-disk)
///
///
///
/// ## Example Usage
///
/// ### Region Disk Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let disk = disk::create(
///         "disk",
///         DiskArgs::builder()
///             .image("debian-cloud/debian-11")
///             .name("my-disk")
///             .size(50)
///             .type_("pd-ssd")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
///     let regiondisk = region_disk::create(
///         "regiondisk",
///         RegionDiskArgs::builder()
///             .name("my-region-disk")
///             .physical_block_size_bytes(4096)
///             .region("us-central1")
///             .replica_zones(vec!["us-central1-a", "us-central1-f",])
///             .snapshot("${snapdisk.id}")
///             .type_("pd-ssd")
///             .build_struct(),
///     );
///     let snapdisk = snapshot::create(
///         "snapdisk",
///         SnapshotArgs::builder()
///             .name("my-snapshot")
///             .source_disk("${disk.name}")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Disk Async
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = region_disk::create(
///         "primary",
///         RegionDiskArgs::builder()
///             .name("primary-region-disk")
///             .physical_block_size_bytes(4096)
///             .region("us-central1")
///             .replica_zones(vec!["us-central1-a", "us-central1-f",])
///             .type_("pd-ssd")
///             .build_struct(),
///     );
///     let secondary = region_disk::create(
///         "secondary",
///         RegionDiskArgs::builder()
///             .async_primary_disk(
///                 RegionDiskAsyncPrimaryDisk::builder()
///                     .disk("${primary.id}")
///                     .build_struct(),
///             )
///             .name("secondary-region-disk")
///             .physical_block_size_bytes(4096)
///             .region("us-east1")
///             .replica_zones(vec!["us-east1-b", "us-east1-c",])
///             .type_("pd-ssd")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Disk Features
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let regiondisk = region_disk::create(
///         "regiondisk",
///         RegionDiskArgs::builder()
///             .guest_os_features(
///                 vec![
///                     RegionDiskGuestOsFeature::builder(). type ("SECURE_BOOT")
///                     .build_struct(), RegionDiskGuestOsFeature::builder(). type
///                     ("MULTI_IP_SUBNET").build_struct(),
///                     RegionDiskGuestOsFeature::builder(). type ("WINDOWS").build_struct(),
///                 ],
///             )
///             .licenses(
///                 vec![
///                     "https://www.googleapis.com/compute/v1/projects/windows-cloud/global/licenses/windows-server-core",
///                 ],
///             )
///             .name("my-region-features-disk")
///             .physical_block_size_bytes(4096)
///             .region("us-central1")
///             .replica_zones(vec!["us-central1-a", "us-central1-f",])
///             .type_("pd-ssd")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RegionDisk can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/disks/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionDisk can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionDisk:RegionDisk default projects/{{project}}/regions/{{region}}/disks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionDisk:RegionDisk default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionDisk:RegionDisk default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionDisk:RegionDisk default {{name}}
/// ```
///
pub mod region_disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionDiskArgs {
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub async_primary_disk: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RegionDiskAsyncPrimaryDisk>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Encrypts the disk using a customer-supplied encryption key.
        /// After you encrypt a disk with a customer-supplied key, you must
        /// provide the same key if you use the disk later (e.g. to create a disk
        /// snapshot or an image, or to attach the disk to a virtual machine).
        /// Customer-supplied encryption keys do not protect access to metadata of
        /// the disk.
        /// If you do not provide an encryption key when creating the disk, then
        /// the disk will be encrypted using an automatically generated key and
        /// you do not need to provide a key to use the disk later.
        /// Structure is documented below.
        #[builder(into, default)]
        pub disk_encryption_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RegionDiskDiskEncryptionKey>,
        >,
        /// A list of features to enable on the guest operating system.
        /// Applicable only for bootable disks.
        /// Structure is documented below.
        #[builder(into, default)]
        pub guest_os_features: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RegionDiskGuestOsFeature>>,
        >,
        /// Specifies the disk interface to use for attaching this disk, which is either SCSI or NVME. The default is SCSI.
        ///
        /// > **Warning:** `interface` is deprecated and will be removed in a future major release. This field is no longer used and can be safely removed from your configurations; disk interfaces are automatically determined on attachment.
        #[builder(into, default)]
        pub interface: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Labels to apply to this disk.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Any applicable license URI.
        #[builder(into, default)]
        pub licenses: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Physical block size of the persistent disk, in bytes. If not present
        /// in a request, a default value is used. Currently supported sizes
        /// are 4096 and 16384, other sizes may be added in the future.
        /// If an unsupported value is requested, the error message will list
        /// the supported values for the caller's project.
        #[builder(into, default)]
        pub physical_block_size_bytes: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A reference to the region where the disk resides.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// URLs of the zones where the disk should be replicated to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub replica_zones: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Size of the persistent disk, specified in GB. You can specify this
        /// field when creating a persistent disk using the sourceImage or
        /// sourceSnapshot parameter, or specify it alone to create an empty
        /// persistent disk.
        /// If you specify this field along with sourceImage or sourceSnapshot,
        /// the value of sizeGb must not be less than the size of the sourceImage
        /// or the size of the snapshot.
        #[builder(into, default)]
        pub size: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The source snapshot used to create this disk. You can provide this as
        /// a partial or full URL to the resource. For example, the following are
        /// valid values:
        /// * `https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot`
        /// * `projects/project/global/snapshots/snapshot`
        /// * `global/snapshots/snapshot`
        #[builder(into, default)]
        pub snapshot: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The source disk used to create this disk. You can provide this as a partial or full URL to the resource.
        /// For example, the following are valid values:
        /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}
        /// * https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}
        /// * projects/{project}/zones/{zone}/disks/{disk}
        /// * projects/{project}/regions/{region}/disks/{disk}
        /// * zones/{zone}/disks/{disk}
        /// * regions/{region}/disks/{disk}
        #[builder(into, default)]
        pub source_disk: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The customer-supplied encryption key of the source snapshot. Required
        /// if the source snapshot is protected by a customer-supplied encryption
        /// key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub source_snapshot_encryption_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RegionDiskSourceSnapshotEncryptionKey>,
        >,
        /// URL of the disk type resource describing which disk type to use to
        /// create the disk. Provide this when creating the disk.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionDiskResult {
        /// A nested object resource.
        /// Structure is documented below.
        pub async_primary_disk: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionDiskAsyncPrimaryDisk>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Encrypts the disk using a customer-supplied encryption key.
        /// After you encrypt a disk with a customer-supplied key, you must
        /// provide the same key if you use the disk later (e.g. to create a disk
        /// snapshot or an image, or to attach the disk to a virtual machine).
        /// Customer-supplied encryption keys do not protect access to metadata of
        /// the disk.
        /// If you do not provide an encryption key when creating the disk, then
        /// the disk will be encrypted using an automatically generated key and
        /// you do not need to provide a key to use the disk later.
        /// Structure is documented below.
        pub disk_encryption_key: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionDiskDiskEncryptionKey>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of features to enable on the guest operating system.
        /// Applicable only for bootable disks.
        /// Structure is documented below.
        pub guest_os_features: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::RegionDiskGuestOsFeature>,
        >,
        /// Specifies the disk interface to use for attaching this disk, which is either SCSI or NVME. The default is SCSI.
        ///
        /// > **Warning:** `interface` is deprecated and will be removed in a future major release. This field is no longer used and can be safely removed from your configurations; disk interfaces are automatically determined on attachment.
        pub interface: pulumi_wasm_rust::Output<Option<String>>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        /// Labels to apply to this disk.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Last attach timestamp in RFC3339 text format.
        pub last_attach_timestamp: pulumi_wasm_rust::Output<String>,
        /// Last detach timestamp in RFC3339 text format.
        pub last_detach_timestamp: pulumi_wasm_rust::Output<String>,
        /// Any applicable license URI.
        pub licenses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Physical block size of the persistent disk, in bytes. If not present
        /// in a request, a default value is used. Currently supported sizes
        /// are 4096 and 16384, other sizes may be added in the future.
        /// If an unsupported value is requested, the error message will list
        /// the supported values for the caller's project.
        pub physical_block_size_bytes: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A reference to the region where the disk resides.
        pub region: pulumi_wasm_rust::Output<String>,
        /// URLs of the zones where the disk should be replicated to.
        ///
        ///
        /// - - -
        pub replica_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Size of the persistent disk, specified in GB. You can specify this
        /// field when creating a persistent disk using the sourceImage or
        /// sourceSnapshot parameter, or specify it alone to create an empty
        /// persistent disk.
        /// If you specify this field along with sourceImage or sourceSnapshot,
        /// the value of sizeGb must not be less than the size of the sourceImage
        /// or the size of the snapshot.
        pub size: pulumi_wasm_rust::Output<i32>,
        /// The source snapshot used to create this disk. You can provide this as
        /// a partial or full URL to the resource. For example, the following are
        /// valid values:
        /// * `https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot`
        /// * `projects/project/global/snapshots/snapshot`
        /// * `global/snapshots/snapshot`
        pub snapshot: pulumi_wasm_rust::Output<Option<String>>,
        /// The source disk used to create this disk. You can provide this as a partial or full URL to the resource.
        /// For example, the following are valid values:
        /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}
        /// * https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}
        /// * projects/{project}/zones/{zone}/disks/{disk}
        /// * projects/{project}/regions/{region}/disks/{disk}
        /// * zones/{zone}/disks/{disk}
        /// * regions/{region}/disks/{disk}
        pub source_disk: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID value of the disk used to create this image. This value may
        /// be used to determine whether the image was taken from the current
        /// or a previous instance of a given disk name.
        pub source_disk_id: pulumi_wasm_rust::Output<String>,
        /// The customer-supplied encryption key of the source snapshot. Required
        /// if the source snapshot is protected by a customer-supplied encryption
        /// key.
        /// Structure is documented below.
        pub source_snapshot_encryption_key: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionDiskSourceSnapshotEncryptionKey>,
        >,
        /// The unique ID of the snapshot used to create this disk. This value
        /// identifies the exact snapshot that was used to create this persistent
        /// disk. For example, if you created the persistent disk from a snapshot
        /// that was later deleted and recreated under the same name, the source
        /// snapshot ID would identify the exact version of the snapshot that was
        /// used.
        pub source_snapshot_id: pulumi_wasm_rust::Output<String>,
        /// URL of the disk type resource describing which disk type to use to
        /// create the disk. Provide this when creating the disk.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Links to the users of the disk (attached instances) in form:
        /// project/zones/zone/instances/instance
        pub users: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionDiskArgs,
    ) -> RegionDiskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let async_primary_disk_binding = args
            .async_primary_disk
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let disk_encryption_key_binding = args
            .disk_encryption_key
            .get_output(context)
            .get_inner();
        let guest_os_features_binding = args
            .guest_os_features
            .get_output(context)
            .get_inner();
        let interface_binding = args.interface.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let licenses_binding = args.licenses.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let physical_block_size_bytes_binding = args
            .physical_block_size_bytes
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let replica_zones_binding = args.replica_zones.get_output(context).get_inner();
        let size_binding = args.size.get_output(context).get_inner();
        let snapshot_binding = args.snapshot.get_output(context).get_inner();
        let source_disk_binding = args.source_disk.get_output(context).get_inner();
        let source_snapshot_encryption_key_binding = args
            .source_snapshot_encryption_key
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionDisk:RegionDisk".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "asyncPrimaryDisk".into(),
                    value: &async_primary_disk_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "diskEncryptionKey".into(),
                    value: &disk_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "guestOsFeatures".into(),
                    value: &guest_os_features_binding,
                },
                register_interface::ObjectField {
                    name: "interface".into(),
                    value: &interface_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "licenses".into(),
                    value: &licenses_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "physicalBlockSizeBytes".into(),
                    value: &physical_block_size_bytes_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "replicaZones".into(),
                    value: &replica_zones_binding,
                },
                register_interface::ObjectField {
                    name: "size".into(),
                    value: &size_binding,
                },
                register_interface::ObjectField {
                    name: "snapshot".into(),
                    value: &snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "sourceDisk".into(),
                    value: &source_disk_binding,
                },
                register_interface::ObjectField {
                    name: "sourceSnapshotEncryptionKey".into(),
                    value: &source_snapshot_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionDiskResult {
            async_primary_disk: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("asyncPrimaryDisk"),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disk_encryption_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("diskEncryptionKey"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            guest_os_features: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("guestOsFeatures"),
            ),
            interface: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("interface"),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            last_attach_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastAttachTimestamp"),
            ),
            last_detach_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastDetachTimestamp"),
            ),
            licenses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("licenses"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            physical_block_size_bytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("physicalBlockSizeBytes"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            replica_zones: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicaZones"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            size: pulumi_wasm_rust::__private::into_domain(o.extract_field("size")),
            snapshot: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshot"),
            ),
            source_disk: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceDisk"),
            ),
            source_disk_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceDiskId"),
            ),
            source_snapshot_encryption_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceSnapshotEncryptionKey"),
            ),
            source_snapshot_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceSnapshotId"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            users: pulumi_wasm_rust::__private::into_domain(o.extract_field("users")),
        }
    }
}
