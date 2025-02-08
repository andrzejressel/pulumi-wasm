#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionInstanceTemplateDisk {
    /// Whether or not the disk should be auto-deleted.
    /// This defaults to true.
    #[builder(into, default)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Box<Option<bool>>,
    /// Indicates that this is a boot disk.
    #[builder(into, default)]
    #[serde(rename = "boot")]
    pub r#boot: Box<Option<bool>>,
    /// A unique device name that is reflected into the
    /// /dev/  tree of a Linux operating system running within the instance. If not
    /// specified, the server chooses a default device name to apply to this disk.
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// Encrypts or decrypts a disk using a customer-supplied encryption key.
    /// 
    /// If you are creating a new disk, this field encrypts the new disk using an encryption key that you provide. If you are attaching an existing disk that is already encrypted, this field decrypts the disk using the customer-supplied encryption key.
    /// 
    /// If you encrypt a disk using a customer-supplied key, you must provide the same key again when you attempt to use this resource at a later time. For example, you must provide the key when you create a snapshot or an image from the disk or when you attach the disk to a virtual machine instance.
    /// 
    /// If you do not provide an encryption key, then the disk will be encrypted using an automatically generated key and you do not need to provide a key to use the disk later.
    /// 
    /// Instance templates do not store customer-supplied encryption keys, so you cannot use your own keys to encrypt disks in a managed instance group. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionKey")]
    pub r#disk_encryption_key: Box<Option<super::super::types::compute::RegionInstanceTemplateDiskDiskEncryptionKey>>,
    /// Name of the disk. When not provided, this defaults
    /// to the name of the instance.
    #[builder(into, default)]
    #[serde(rename = "diskName")]
    pub r#disk_name: Box<Option<String>>,
    /// The size of the image in gigabytes. If not
    /// specified, it will inherit the size of its base image. For SCRATCH disks,
    /// the size must be exactly 375GB.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// The GCE disk type. Such as `"pd-ssd"`, `"local-ssd"`,
    /// `"pd-balanced"` or `"pd-standard"`.
    #[builder(into, default)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<Option<String>>,
    /// Specifies the disk interface to use for attaching this disk,
    /// which is either SCSI or NVME. The default is SCSI. Persistent disks must always use SCSI
    /// and the request will fail if you attempt to attach a persistent disk in any other format
    /// than SCSI. Local SSDs can use either NVME or SCSI.
    #[builder(into, default)]
    #[serde(rename = "interface")]
    pub r#interface: Box<Option<String>>,
    /// A set of ket/value label pairs to assign to disk created from
    /// this template
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// The mode in which to attach this disk, either READ_WRITE
    /// or READ_ONLY. If you are attaching or creating a boot disk, this must
    /// read-write mode.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Indicates how many IOPS to provision for the disk. This
    /// sets the number of I/O operations per second that the disk can handle.
    /// Values must be between 10,000 and 120,000. For more details, see the
    /// [Extreme persistent disk documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk).
    #[builder(into, default)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: Box<Option<i32>>,
    /// Indicates how much throughput to provision for the disk, in MB/s. This sets the amount of data that can be read or written from the disk per second. Values must greater than or equal to 1. For more details, see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks).
    #[builder(into, default)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: Box<Option<i32>>,
    /// A set of key/value resource manager tag pairs to bind to this disk. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456.
    #[builder(into, default)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// - A list (short name or id) of resource policies to attach to this disk for automatic snapshot creations. Currently a max of 1 resource policy is supported.
    #[builder(into, default)]
    #[serde(rename = "resourcePolicies")]
    pub r#resource_policies: Box<Option<String>>,
    /// The name (**not self_link**)
    /// of the disk (such as those managed by `gcp.compute.Disk`) to attach.
    /// > **Note:** Either `source`, `source_image`, or `source_snapshot` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// The image from which to
    /// initialize this disk. This can be one of: the image's `self_link`,
    /// `projects/{project}/global/images/{image}`,
    /// `projects/{project}/global/images/family/{family}`, `global/images/{image}`,
    /// `global/images/family/{family}`, `family/{family}`, `{project}/{family}`,
    /// `{project}/{image}`, `{family}`, or `{image}`.
    /// > **Note:** Either `source`, `source_image`, or `source_snapshot` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into, default)]
    #[serde(rename = "sourceImage")]
    pub r#source_image: Box<Option<String>>,
    /// The customer-supplied encryption
    /// key of the source image. Required if the source image is protected by a
    /// customer-supplied encryption key.
    /// 
    /// Instance templates do not store customer-supplied encryption keys, so you
    /// cannot create disks for instances in a managed instance group if the source
    /// images are encrypted with your own keys. Structure
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "sourceImageEncryptionKey")]
    pub r#source_image_encryption_key: Box<Option<super::super::types::compute::RegionInstanceTemplateDiskSourceImageEncryptionKey>>,
    /// The source snapshot to create this disk.
    /// > **Note:** Either `source`, `source_image`, or `source_snapshot` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into, default)]
    #[serde(rename = "sourceSnapshot")]
    pub r#source_snapshot: Box<Option<String>>,
    /// The customer-supplied encryption
    /// key of the source snapshot. Structure
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "sourceSnapshotEncryptionKey")]
    pub r#source_snapshot_encryption_key: Box<Option<super::super::types::compute::RegionInstanceTemplateDiskSourceSnapshotEncryptionKey>>,
    /// The type of GCE disk, can be either `"SCRATCH"` or
    /// `"PERSISTENT"`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
