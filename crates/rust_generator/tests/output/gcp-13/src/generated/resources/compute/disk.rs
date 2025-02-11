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
/// To get more information about Disk, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/disks)
/// * How-to Guides
///     * [Adding a persistent disk](https://cloud.google.com/compute/docs/disks/add-persistent-disk)
///
///
/// ## Example Usage
///
/// ### Disk Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Disk
///     properties:
///       name: test-disk
///       type: pd-ssd
///       zone: us-central1-a
///       image: debian-11-bullseye-v20220719
///       labels:
///         environment: dev
///       physicalBlockSizeBytes: 4096
/// ```
/// ### Disk Async
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = disk::create(
///         "primary",
///         DiskArgs::builder()
///             .name("async-test-disk")
///             .physical_block_size_bytes(4096)
///             .type_("pd-ssd")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
///     let secondary = disk::create(
///         "secondary",
///         DiskArgs::builder()
///             .async_primary_disk(
///                 DiskAsyncPrimaryDisk::builder().disk("${primary.id}").build_struct(),
///             )
///             .name("async-secondary-test-disk")
///             .physical_block_size_bytes(4096)
///             .type_("pd-ssd")
///             .zone("us-east1-c")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Disk Features
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Disk
///     properties:
///       name: test-disk-features
///       type: pd-ssd
///       zone: us-central1-a
///       labels:
///         environment: dev
///       guestOsFeatures:
///         - type: SECURE_BOOT
///         - type: MULTI_IP_SUBNET
///         - type: WINDOWS
///       licenses:
///         - https://www.googleapis.com/compute/v1/projects/windows-cloud/global/licenses/windows-server-core
///       physicalBlockSizeBytes: 4096
/// ```
///
/// ## Import
///
/// Disk can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/disks/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Disk can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/disk:Disk default projects/{{project}}/zones/{{zone}}/disks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/disk:Disk default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/disk:Disk default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/disk:Disk default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiskArgs {
        /// The accessMode of the disk.
        /// For example:
        /// * READ_WRITE_SINGLE
        /// * READ_WRITE_MANY
        /// * READ_ONLY_SINGLE
        #[builder(into, default)]
        pub access_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub async_primary_disk: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::DiskAsyncPrimaryDisk>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        pub disk_encryption_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::DiskDiskEncryptionKey>,
        >,
        /// Whether this disk is using confidential compute mode.
        /// Note: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true
        #[builder(into, default)]
        pub enable_confidential_compute: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A list of features to enable on the guest operating system.
        /// Applicable only for bootable disks.
        /// Structure is documented below.
        #[builder(into, default)]
        pub guest_os_features: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::DiskGuestOsFeature>>,
        >,
        /// The image from which to initialize this disk. This can be
        /// one of: the image's `self_link`, `projects/{project}/global/images/{image}`,
        /// `projects/{project}/global/images/family/{family}`, `global/images/{image}`,
        /// `global/images/family/{family}`, `family/{family}`, `{project}/{family}`,
        /// `{project}/{image}`, `{family}`, or `{image}`. If referred by family, the
        /// images names must include the family name. If they don't, use the
        /// [gcp.compute.Image data source](https://www.terraform.io/docs/providers/google/d/compute_image.html).
        /// For instance, the image `centos-6-v20180104` includes its family name `centos-6`.
        /// These images can be referred by family name here.
        #[builder(into, default)]
        pub image: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the disk interface to use for attaching this disk, which is either SCSI or NVME. The default is SCSI.
        ///
        /// > **Warning:** `interface` is deprecated and will be removed in a future major release. This field is no longer used and can be safely removed from your configurations; disk interfaces are automatically determined on attachment.
        #[builder(into, default)]
        pub interface: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels to apply to this disk.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Any applicable license URI.
        #[builder(into, default)]
        pub licenses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Indicates whether or not the disk can be read/write attached to more than one instance.
        #[builder(into, default)]
        pub multi_writer: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Physical block size of the persistent disk, in bytes. If not present
        /// in a request, a default value is used. Currently supported sizes
        /// are 4096 and 16384, other sizes may be added in the future.
        /// If an unsupported value is requested, the error message will list
        /// the supported values for the caller's project.
        #[builder(into, default)]
        pub physical_block_size_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates how many IOPS must be provisioned for the disk.
        /// Note: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk
        /// allows for an update of IOPS every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it
        #[builder(into, default)]
        pub provisioned_iops: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Indicates how much Throughput must be provisioned for the disk.
        /// Note: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk
        /// allows for an update of Throughput every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it
        #[builder(into, default)]
        pub provisioned_throughput: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Resource policies applied to this disk for automatic snapshot creations.
        /// ~>**NOTE** This value does not support updating the
        /// resource policy, as resource policies can not be updated more than
        /// one at a time. Use
        /// `gcp.compute.DiskResourcePolicyAttachment`
        /// to allow for updating the resource policy attached to the disk.
        #[builder(into, default)]
        pub resource_policies: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Size of the persistent disk, specified in GB. You can specify this
        /// field when creating a persistent disk using the `image` or
        /// `snapshot` parameter, or specify it alone to create an empty
        /// persistent disk.
        /// If you specify this field along with `image` or `snapshot`,
        /// the value must not be less than the size of the image
        /// or the size of the snapshot.
        /// ~>**NOTE** If you change the size, the provider updates the disk size
        /// if upsizing is detected but recreates the disk if downsizing is requested.
        /// You can add `lifecycle.prevent_destroy` in the config to prevent destroying
        /// and recreating.
        #[builder(into, default)]
        pub size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The source snapshot used to create this disk. You can provide this as
        /// a partial or full URL to the resource. If the snapshot is in another
        /// project than this disk, you must supply a full URL. For example, the
        /// following are valid values:
        /// * `https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot`
        /// * `projects/project/global/snapshots/snapshot`
        /// * `global/snapshots/snapshot`
        #[builder(into, default)]
        pub snapshot: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source disk used to create this disk. You can provide this as a partial or full URL to the resource.
        /// For example, the following are valid values:
        /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}
        /// * https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}
        /// * projects/{project}/zones/{zone}/disks/{disk}
        /// * projects/{project}/regions/{region}/disks/{disk}
        /// * zones/{zone}/disks/{disk}
        /// * regions/{region}/disks/{disk}
        #[builder(into, default)]
        pub source_disk: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The customer-supplied encryption key of the source image. Required if
        /// the source image is protected by a customer-supplied encryption key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub source_image_encryption_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::DiskSourceImageEncryptionKey>,
        >,
        /// The customer-supplied encryption key of the source snapshot. Required
        /// if the source snapshot is protected by a customer-supplied encryption
        /// key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub source_snapshot_encryption_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::DiskSourceSnapshotEncryptionKey>,
        >,
        /// The URL or the name of the storage pool in which the new disk is created.
        /// For example:
        /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}
        /// * /projects/{project}/zones/{zone}/storagePools/{storagePool}
        /// * /zones/{zone}/storagePools/{storagePool}
        /// * /{storagePool}
        #[builder(into, default)]
        pub storage_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of the disk type resource describing which disk type to use to
        /// create the disk. Provide this when creating the disk.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the zone where the disk resides.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DiskResult {
        /// The accessMode of the disk.
        /// For example:
        /// * READ_WRITE_SINGLE
        /// * READ_WRITE_MANY
        /// * READ_ONLY_SINGLE
        pub access_mode: pulumi_gestalt_rust::Output<String>,
        /// A nested object resource.
        /// Structure is documented below.
        pub async_primary_disk: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::DiskAsyncPrimaryDisk>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
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
        pub disk_encryption_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::DiskDiskEncryptionKey>,
        >,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub disk_id: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether this disk is using confidential compute mode.
        /// Note: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true
        pub enable_confidential_compute: pulumi_gestalt_rust::Output<bool>,
        /// A list of features to enable on the guest operating system.
        /// Applicable only for bootable disks.
        /// Structure is documented below.
        pub guest_os_features: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::DiskGuestOsFeature>,
        >,
        /// The image from which to initialize this disk. This can be
        /// one of: the image's `self_link`, `projects/{project}/global/images/{image}`,
        /// `projects/{project}/global/images/family/{family}`, `global/images/{image}`,
        /// `global/images/family/{family}`, `family/{family}`, `{project}/{family}`,
        /// `{project}/{image}`, `{family}`, or `{image}`. If referred by family, the
        /// images names must include the family name. If they don't, use the
        /// [gcp.compute.Image data source](https://www.terraform.io/docs/providers/google/d/compute_image.html).
        /// For instance, the image `centos-6-v20180104` includes its family name `centos-6`.
        /// These images can be referred by family name here.
        pub image: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the disk interface to use for attaching this disk, which is either SCSI or NVME. The default is SCSI.
        ///
        /// > **Warning:** `interface` is deprecated and will be removed in a future major release. This field is no longer used and can be safely removed from your configurations; disk interfaces are automatically determined on attachment.
        pub interface: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Labels to apply to this disk.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Last attach timestamp in RFC3339 text format.
        pub last_attach_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Last detach timestamp in RFC3339 text format.
        pub last_detach_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Any applicable license URI.
        pub licenses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Indicates whether or not the disk can be read/write attached to more than one instance.
        pub multi_writer: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Physical block size of the persistent disk, in bytes. If not present
        /// in a request, a default value is used. Currently supported sizes
        /// are 4096 and 16384, other sizes may be added in the future.
        /// If an unsupported value is requested, the error message will list
        /// the supported values for the caller's project.
        pub physical_block_size_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Indicates how many IOPS must be provisioned for the disk.
        /// Note: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk
        /// allows for an update of IOPS every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it
        pub provisioned_iops: pulumi_gestalt_rust::Output<i32>,
        /// Indicates how much Throughput must be provisioned for the disk.
        /// Note: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk
        /// allows for an update of Throughput every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it
        pub provisioned_throughput: pulumi_gestalt_rust::Output<i32>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource policies applied to this disk for automatic snapshot creations.
        /// ~>**NOTE** This value does not support updating the
        /// resource policy, as resource policies can not be updated more than
        /// one at a time. Use
        /// `gcp.compute.DiskResourcePolicyAttachment`
        /// to allow for updating the resource policy attached to the disk.
        pub resource_policies: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Size of the persistent disk, specified in GB. You can specify this
        /// field when creating a persistent disk using the `image` or
        /// `snapshot` parameter, or specify it alone to create an empty
        /// persistent disk.
        /// If you specify this field along with `image` or `snapshot`,
        /// the value must not be less than the size of the image
        /// or the size of the snapshot.
        /// ~>**NOTE** If you change the size, the provider updates the disk size
        /// if upsizing is detected but recreates the disk if downsizing is requested.
        /// You can add `lifecycle.prevent_destroy` in the config to prevent destroying
        /// and recreating.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// The source snapshot used to create this disk. You can provide this as
        /// a partial or full URL to the resource. If the snapshot is in another
        /// project than this disk, you must supply a full URL. For example, the
        /// following are valid values:
        /// * `https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot`
        /// * `projects/project/global/snapshots/snapshot`
        /// * `global/snapshots/snapshot`
        pub snapshot: pulumi_gestalt_rust::Output<Option<String>>,
        /// The source disk used to create this disk. You can provide this as a partial or full URL to the resource.
        /// For example, the following are valid values:
        /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}
        /// * https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}
        /// * projects/{project}/zones/{zone}/disks/{disk}
        /// * projects/{project}/regions/{region}/disks/{disk}
        /// * zones/{zone}/disks/{disk}
        /// * regions/{region}/disks/{disk}
        pub source_disk: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID value of the disk used to create this image. This value may
        /// be used to determine whether the image was taken from the current
        /// or a previous instance of a given disk name.
        pub source_disk_id: pulumi_gestalt_rust::Output<String>,
        /// The customer-supplied encryption key of the source image. Required if
        /// the source image is protected by a customer-supplied encryption key.
        /// Structure is documented below.
        pub source_image_encryption_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::DiskSourceImageEncryptionKey>,
        >,
        /// The ID value of the image used to create this disk. This value
        /// identifies the exact image that was used to create this persistent
        /// disk. For example, if you created the persistent disk from an image
        /// that was later deleted and recreated under the same name, the source
        /// image ID would identify the exact version of the image that was used.
        pub source_image_id: pulumi_gestalt_rust::Output<String>,
        /// The customer-supplied encryption key of the source snapshot. Required
        /// if the source snapshot is protected by a customer-supplied encryption
        /// key.
        /// Structure is documented below.
        pub source_snapshot_encryption_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::DiskSourceSnapshotEncryptionKey>,
        >,
        /// The unique ID of the snapshot used to create this disk. This value
        /// identifies the exact snapshot that was used to create this persistent
        /// disk. For example, if you created the persistent disk from a snapshot
        /// that was later deleted and recreated under the same name, the source
        /// snapshot ID would identify the exact version of the snapshot that was
        /// used.
        pub source_snapshot_id: pulumi_gestalt_rust::Output<String>,
        /// The URL or the name of the storage pool in which the new disk is created.
        /// For example:
        /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}
        /// * /projects/{project}/zones/{zone}/storagePools/{storagePool}
        /// * /zones/{zone}/storagePools/{storagePool}
        /// * /{storagePool}
        pub storage_pool: pulumi_gestalt_rust::Output<Option<String>>,
        /// URL of the disk type resource describing which disk type to use to
        /// create the disk. Provide this when creating the disk.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// Links to the users of the disk (attached instances) in form:
        /// project/zones/zone/instances/instance
        pub users: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A reference to the zone where the disk resides.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DiskArgs,
    ) -> DiskResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_mode_binding = args.access_mode.get_output(context);
        let async_primary_disk_binding = args.async_primary_disk.get_output(context);
        let description_binding = args.description.get_output(context);
        let disk_encryption_key_binding = args.disk_encryption_key.get_output(context);
        let enable_confidential_compute_binding = args
            .enable_confidential_compute
            .get_output(context);
        let guest_os_features_binding = args.guest_os_features.get_output(context);
        let image_binding = args.image.get_output(context);
        let interface_binding = args.interface.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let licenses_binding = args.licenses.get_output(context);
        let multi_writer_binding = args.multi_writer.get_output(context);
        let name_binding = args.name.get_output(context);
        let physical_block_size_bytes_binding = args
            .physical_block_size_bytes
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let provisioned_iops_binding = args.provisioned_iops.get_output(context);
        let provisioned_throughput_binding = args
            .provisioned_throughput
            .get_output(context);
        let resource_policies_binding = args.resource_policies.get_output(context);
        let size_binding = args.size.get_output(context);
        let snapshot_binding = args.snapshot.get_output(context);
        let source_disk_binding = args.source_disk.get_output(context);
        let source_image_encryption_key_binding = args
            .source_image_encryption_key
            .get_output(context);
        let source_snapshot_encryption_key_binding = args
            .source_snapshot_encryption_key
            .get_output(context);
        let storage_pool_binding = args.storage_pool.get_output(context);
        let type__binding = args.type_.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/disk:Disk".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessMode".into(),
                    value: &access_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "asyncPrimaryDisk".into(),
                    value: &async_primary_disk_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskEncryptionKey".into(),
                    value: &disk_encryption_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableConfidentialCompute".into(),
                    value: &enable_confidential_compute_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guestOsFeatures".into(),
                    value: &guest_os_features_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "image".into(),
                    value: &image_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interface".into(),
                    value: &interface_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenses".into(),
                    value: &licenses_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiWriter".into(),
                    value: &multi_writer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "physicalBlockSizeBytes".into(),
                    value: &physical_block_size_bytes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisionedIops".into(),
                    value: &provisioned_iops_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisionedThroughput".into(),
                    value: &provisioned_throughput_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourcePolicies".into(),
                    value: &resource_policies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "size".into(),
                    value: &size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshot".into(),
                    value: &snapshot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceDisk".into(),
                    value: &source_disk_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceImageEncryptionKey".into(),
                    value: &source_image_encryption_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceSnapshotEncryptionKey".into(),
                    value: &source_snapshot_encryption_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storagePool".into(),
                    value: &storage_pool_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DiskResult {
            access_mode: o.get_field("accessMode"),
            async_primary_disk: o.get_field("asyncPrimaryDisk"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            disk_encryption_key: o.get_field("diskEncryptionKey"),
            disk_id: o.get_field("diskId"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_confidential_compute: o.get_field("enableConfidentialCompute"),
            guest_os_features: o.get_field("guestOsFeatures"),
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
            source_image_encryption_key: o.get_field("sourceImageEncryptionKey"),
            source_image_id: o.get_field("sourceImageId"),
            source_snapshot_encryption_key: o.get_field("sourceSnapshotEncryptionKey"),
            source_snapshot_id: o.get_field("sourceSnapshotId"),
            storage_pool: o.get_field("storagePool"),
            type_: o.get_field("type"),
            users: o.get_field("users"),
            zone: o.get_field("zone"),
        }
    }
}
