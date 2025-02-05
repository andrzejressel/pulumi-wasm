#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionInstanceTemplateDisk {
    /// Whether or not the disk should be auto-deleted.
    /// This defaults to true.
    #[builder(into)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Box<bool>,
    /// Indicates that this is a boot disk.
    #[builder(into)]
    #[serde(rename = "boot")]
    pub r#boot: Box<bool>,
    /// A unique device name that is reflected into the
    /// /dev/  tree of a Linux operating system running within the instance. If not
    /// specified, the server chooses a default device name to apply to this disk.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// Encrypts or decrypts a disk using a customer-supplied encryption key.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeys")]
    pub r#disk_encryption_keys: Box<Vec<super::super::types::compute::GetRegionInstanceTemplateDiskDiskEncryptionKey>>,
    /// Name of the disk. When not provided, this defaults
    /// to the name of the instance.
    #[builder(into)]
    #[serde(rename = "diskName")]
    pub r#disk_name: Box<String>,
    /// The size of the image in gigabytes. If not
    /// specified, it will inherit the size of its base image. For SCRATCH disks,
    /// the size must be exactly 375GB.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<i32>,
    /// The GCE disk type. Such as `"pd-ssd"`, `"local-ssd"`,
    /// `"pd-balanced"` or `"pd-standard"`.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<String>,
    /// Specifies the disk interface to use for attaching this disk,
    /// which is either SCSI or NVME. The default is SCSI. Persistent disks must always use SCSI
    /// and the request will fail if you attempt to attach a persistent disk in any other format
    /// than SCSI. Local SSDs can use either NVME or SCSI.
    #[builder(into)]
    #[serde(rename = "interface")]
    pub r#interface: Box<String>,
    /// (Optional) A set of ket/value label pairs to assign to disk created from
    /// this template
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The mode in which to attach this disk, either READ_WRITE
    /// or READ_ONLY. If you are attaching or creating a boot disk, this must
    /// read-write mode.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Indicates how many IOPS to provision for the disk. This
    /// sets the number of I/O operations per second that the disk can handle.
    /// Values must be between 10,000 and 120,000. For more details, see the
    /// [Extreme persistent disk documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk).
    #[builder(into)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: Box<i32>,
    /// Indicates how much throughput to provision for the disk, in MB/s. This sets the amount of data that can be read or written from the disk per second. Values must greater than or equal to 1. For more details, see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks).
    #[builder(into)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: Box<i32>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<std::collections::HashMap<String, String>>,
    /// (Optional) -- A list of short names of resource policies to attach to this disk for automatic snapshot creations. Currently a max of 1 resource policy is supported.
    #[builder(into)]
    #[serde(rename = "resourcePolicies")]
    pub r#resource_policies: Box<Vec<String>>,
    /// The name (**not self_link**)
    /// of the disk (such as those managed by `gcp.compute.Disk`) to attach.
    /// > **Note:** Either `source` or `source_image` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
    /// The image from which to
    /// initialize this disk. This can be one of: the image's `self_link`,
    /// `projects/{project}/global/images/{image}`,
    /// `projects/{project}/global/images/family/{family}`, `global/images/{image}`,
    /// `global/images/family/{family}`, `family/{family}`, `{project}/{family}`,
    /// `{project}/{image}`, `{family}`, or `{image}`.
    /// > **Note:** Either `source` or `source_image` is **required** in a disk block unless the disk type is `local-ssd`. Check the API [docs](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/insert) for details.
    #[builder(into)]
    #[serde(rename = "sourceImage")]
    pub r#source_image: Box<String>,
    /// The customer-supplied encryption key of the source
    /// image. Required if the source image is protected by a
    /// customer-supplied encryption key.
    /// 
    /// Instance templates do not store customer-supplied
    /// encryption keys, so you cannot create disks for
    /// instances in a managed instance group if the source
    /// images are encrypted with your own keys.
    #[builder(into)]
    #[serde(rename = "sourceImageEncryptionKeys")]
    pub r#source_image_encryption_keys: Box<Vec<super::super::types::compute::GetRegionInstanceTemplateDiskSourceImageEncryptionKey>>,
    /// The source snapshot to create this disk. When creating
    /// a new instance, one of initializeParams.sourceSnapshot,
    /// initializeParams.sourceImage, or disks.source is
    /// required except for local SSD.
    #[builder(into)]
    #[serde(rename = "sourceSnapshot")]
    pub r#source_snapshot: Box<String>,
    /// The customer-supplied encryption key of the source snapshot.
    #[builder(into)]
    #[serde(rename = "sourceSnapshotEncryptionKeys")]
    pub r#source_snapshot_encryption_keys: Box<Vec<super::super::types::compute::GetRegionInstanceTemplateDiskSourceSnapshotEncryptionKey>>,
    /// The accelerator type resource to expose to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
