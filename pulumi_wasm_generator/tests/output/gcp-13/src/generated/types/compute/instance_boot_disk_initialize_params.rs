#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceBootDiskInitializeParams {
    /// Whether this disk is using confidential compute mode.
    /// Note: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true.
    #[builder(into, default)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: Box<Option<bool>>,
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
    #[serde(rename = "image")]
    pub r#image: Box<Option<String>>,
    /// A set of key/value label pairs assigned to the disk. This
    /// field is only applicable for persistent disks.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Indicates how many IOPS to provision for the disk.
    /// This sets the number of I/O operations per second that the disk can handle.
    /// For more details,see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks).
    /// Note: Updating currently is only supported for hyperdisk skus via disk update
    /// api/gcloud without the need to delete and recreate the disk, hyperdisk allows
    /// for an update of IOPS every 4 hours. To update your hyperdisk more frequently,
    /// you'll need to manually delete and recreate it.
    #[builder(into, default)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: Box<Option<i32>>,
    /// Indicates how much throughput to provision for the disk.
    /// This sets the number of throughput mb per second that the disk can handle.
    /// For more details,see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks).
    /// Note: Updating currently is only supported for hyperdisk skus via disk update
    /// api/gcloud without the need to delete and recreate the disk, hyperdisk allows
    /// for an update of throughput every 4 hours. To update your hyperdisk more
    /// frequently, you'll need to manually delete and recreate it.
    #[builder(into, default)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: Box<Option<i32>>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into, default)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// A list of self_links of resource policies to attach to the instance's boot disk. Modifying this list will cause the instance to recreate. Currently a max of 1 resource policy is supported.
    #[builder(into, default)]
    #[serde(rename = "resourcePolicies")]
    pub r#resource_policies: Box<Option<String>>,
    /// The size of the image in gigabytes. If not specified, it
    /// will inherit the size of its base image.
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<i32>>,
    /// The URL or the name of the storage pool in which the new disk is created.
    /// For example:
    /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}
    /// * /projects/{project}/zones/{zone}/storagePools/{storagePool}
    /// * /zones/{zone}/storagePools/{storagePool}
    /// * /{storagePool}
    #[builder(into, default)]
    #[serde(rename = "storagePool")]
    pub r#storage_pool: Box<Option<String>>,
    /// The GCE disk type. Such as pd-standard, pd-balanced or pd-ssd.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
