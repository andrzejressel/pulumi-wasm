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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiskArgs {
        /// The accessMode of the disk.
        /// For example:
        /// * READ_WRITE_SINGLE
        /// * READ_WRITE_MANY
        /// * READ_ONLY_SINGLE
        #[builder(into, default)]
        pub access_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub async_primary_disk: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::DiskAsyncPrimaryDisk>,
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
            Option<super::super::types::compute::DiskDiskEncryptionKey>,
        >,
        /// Whether this disk is using confidential compute mode.
        /// Note: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true
        #[builder(into, default)]
        pub enable_confidential_compute: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A list of features to enable on the guest operating system.
        /// Applicable only for bootable disks.
        /// Structure is documented below.
        #[builder(into, default)]
        pub guest_os_features: pulumi_wasm_rust::InputOrOutput<
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
        pub image: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
        /// Indicates whether or not the disk can be read/write attached to more than one instance.
        #[builder(into, default)]
        pub multi_writer: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
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
        /// Indicates how many IOPS must be provisioned for the disk.
        /// Note: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk
        /// allows for an update of IOPS every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it
        #[builder(into, default)]
        pub provisioned_iops: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Indicates how much Throughput must be provisioned for the disk.
        /// Note: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk
        /// allows for an update of Throughput every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it
        #[builder(into, default)]
        pub provisioned_throughput: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Resource policies applied to this disk for automatic snapshot creations.
        /// ~>**NOTE** This value does not support updating the
        /// resource policy, as resource policies can not be updated more than
        /// one at a time. Use
        /// `gcp.compute.DiskResourcePolicyAttachment`
        /// to allow for updating the resource policy attached to the disk.
        #[builder(into, default)]
        pub resource_policies: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
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
        pub size: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The source snapshot used to create this disk. You can provide this as
        /// a partial or full URL to the resource. If the snapshot is in another
        /// project than this disk, you must supply a full URL. For example, the
        /// following are valid values:
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
        /// The customer-supplied encryption key of the source image. Required if
        /// the source image is protected by a customer-supplied encryption key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub source_image_encryption_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::DiskSourceImageEncryptionKey>,
        >,
        /// The customer-supplied encryption key of the source snapshot. Required
        /// if the source snapshot is protected by a customer-supplied encryption
        /// key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub source_snapshot_encryption_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::DiskSourceSnapshotEncryptionKey>,
        >,
        /// The URL or the name of the storage pool in which the new disk is created.
        /// For example:
        /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}
        /// * /projects/{project}/zones/{zone}/storagePools/{storagePool}
        /// * /zones/{zone}/storagePools/{storagePool}
        /// * /{storagePool}
        #[builder(into, default)]
        pub storage_pool: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// URL of the disk type resource describing which disk type to use to
        /// create the disk. Provide this when creating the disk.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A reference to the zone where the disk resides.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DiskResult {
        /// The accessMode of the disk.
        /// For example:
        /// * READ_WRITE_SINGLE
        /// * READ_WRITE_MANY
        /// * READ_ONLY_SINGLE
        pub access_mode: pulumi_wasm_rust::Output<String>,
        /// A nested object resource.
        /// Structure is documented below.
        pub async_primary_disk: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::DiskAsyncPrimaryDisk>,
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
            Option<super::super::types::compute::DiskDiskEncryptionKey>,
        >,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub disk_id: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether this disk is using confidential compute mode.
        /// Note: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true
        pub enable_confidential_compute: pulumi_wasm_rust::Output<bool>,
        /// A list of features to enable on the guest operating system.
        /// Applicable only for bootable disks.
        /// Structure is documented below.
        pub guest_os_features: pulumi_wasm_rust::Output<
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
        pub image: pulumi_wasm_rust::Output<Option<String>>,
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
        /// Indicates whether or not the disk can be read/write attached to more than one instance.
        pub multi_writer: pulumi_wasm_rust::Output<Option<bool>>,
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
        /// Indicates how many IOPS must be provisioned for the disk.
        /// Note: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk
        /// allows for an update of IOPS every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it
        pub provisioned_iops: pulumi_wasm_rust::Output<i32>,
        /// Indicates how much Throughput must be provisioned for the disk.
        /// Note: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk
        /// allows for an update of Throughput every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it
        pub provisioned_throughput: pulumi_wasm_rust::Output<i32>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource policies applied to this disk for automatic snapshot creations.
        /// ~>**NOTE** This value does not support updating the
        /// resource policy, as resource policies can not be updated more than
        /// one at a time. Use
        /// `gcp.compute.DiskResourcePolicyAttachment`
        /// to allow for updating the resource policy attached to the disk.
        pub resource_policies: pulumi_wasm_rust::Output<Vec<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
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
        pub size: pulumi_wasm_rust::Output<i32>,
        /// The source snapshot used to create this disk. You can provide this as
        /// a partial or full URL to the resource. If the snapshot is in another
        /// project than this disk, you must supply a full URL. For example, the
        /// following are valid values:
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
        /// The customer-supplied encryption key of the source image. Required if
        /// the source image is protected by a customer-supplied encryption key.
        /// Structure is documented below.
        pub source_image_encryption_key: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::DiskSourceImageEncryptionKey>,
        >,
        /// The ID value of the image used to create this disk. This value
        /// identifies the exact image that was used to create this persistent
        /// disk. For example, if you created the persistent disk from an image
        /// that was later deleted and recreated under the same name, the source
        /// image ID would identify the exact version of the image that was used.
        pub source_image_id: pulumi_wasm_rust::Output<String>,
        /// The customer-supplied encryption key of the source snapshot. Required
        /// if the source snapshot is protected by a customer-supplied encryption
        /// key.
        /// Structure is documented below.
        pub source_snapshot_encryption_key: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::DiskSourceSnapshotEncryptionKey>,
        >,
        /// The unique ID of the snapshot used to create this disk. This value
        /// identifies the exact snapshot that was used to create this persistent
        /// disk. For example, if you created the persistent disk from a snapshot
        /// that was later deleted and recreated under the same name, the source
        /// snapshot ID would identify the exact version of the snapshot that was
        /// used.
        pub source_snapshot_id: pulumi_wasm_rust::Output<String>,
        /// The URL or the name of the storage pool in which the new disk is created.
        /// For example:
        /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}
        /// * /projects/{project}/zones/{zone}/storagePools/{storagePool}
        /// * /zones/{zone}/storagePools/{storagePool}
        /// * /{storagePool}
        pub storage_pool: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of the disk type resource describing which disk type to use to
        /// create the disk. Provide this when creating the disk.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Links to the users of the disk (attached instances) in form:
        /// project/zones/zone/instances/instance
        pub users: pulumi_wasm_rust::Output<Vec<String>>,
        /// A reference to the zone where the disk resides.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DiskArgs,
    ) -> DiskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_mode_binding = args.access_mode.get_output(context).get_inner();
        let async_primary_disk_binding = args
            .async_primary_disk
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let disk_encryption_key_binding = args
            .disk_encryption_key
            .get_output(context)
            .get_inner();
        let enable_confidential_compute_binding = args
            .enable_confidential_compute
            .get_output(context)
            .get_inner();
        let guest_os_features_binding = args
            .guest_os_features
            .get_output(context)
            .get_inner();
        let image_binding = args.image.get_output(context).get_inner();
        let interface_binding = args.interface.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let licenses_binding = args.licenses.get_output(context).get_inner();
        let multi_writer_binding = args.multi_writer.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let physical_block_size_bytes_binding = args
            .physical_block_size_bytes
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let provisioned_iops_binding = args
            .provisioned_iops
            .get_output(context)
            .get_inner();
        let provisioned_throughput_binding = args
            .provisioned_throughput
            .get_output(context)
            .get_inner();
        let resource_policies_binding = args
            .resource_policies
            .get_output(context)
            .get_inner();
        let size_binding = args.size.get_output(context).get_inner();
        let snapshot_binding = args.snapshot.get_output(context).get_inner();
        let source_disk_binding = args.source_disk.get_output(context).get_inner();
        let source_image_encryption_key_binding = args
            .source_image_encryption_key
            .get_output(context)
            .get_inner();
        let source_snapshot_encryption_key_binding = args
            .source_snapshot_encryption_key
            .get_output(context)
            .get_inner();
        let storage_pool_binding = args.storage_pool.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/disk:Disk".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessMode".into(),
                    value: &access_mode_binding,
                },
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
                    name: "enableConfidentialCompute".into(),
                    value: &enable_confidential_compute_binding,
                },
                register_interface::ObjectField {
                    name: "guestOsFeatures".into(),
                    value: &guest_os_features_binding,
                },
                register_interface::ObjectField {
                    name: "image".into(),
                    value: &image_binding,
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
                    name: "multiWriter".into(),
                    value: &multi_writer_binding,
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
                    name: "provisionedIops".into(),
                    value: &provisioned_iops_binding,
                },
                register_interface::ObjectField {
                    name: "provisionedThroughput".into(),
                    value: &provisioned_throughput_binding,
                },
                register_interface::ObjectField {
                    name: "resourcePolicies".into(),
                    value: &resource_policies_binding,
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
                    name: "sourceImageEncryptionKey".into(),
                    value: &source_image_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "sourceSnapshotEncryptionKey".into(),
                    value: &source_snapshot_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "storagePool".into(),
                    value: &storage_pool_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
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
                    name: "asyncPrimaryDisk".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "diskEncryptionKey".into(),
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
                    name: "sourceImageEncryptionKey".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageId".into(),
                },
                register_interface::ResultField {
                    name: "sourceSnapshotEncryptionKey".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DiskResult {
            access_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessMode").unwrap(),
            ),
            async_primary_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("asyncPrimaryDisk").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disk_encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskEncryptionKey").unwrap(),
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
            source_image_encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageEncryptionKey").unwrap(),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageId").unwrap(),
            ),
            source_snapshot_encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSnapshotEncryptionKey").unwrap(),
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
