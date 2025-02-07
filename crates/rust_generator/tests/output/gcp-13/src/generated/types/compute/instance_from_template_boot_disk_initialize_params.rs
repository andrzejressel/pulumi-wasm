#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceFromTemplateBootDiskInitializeParams {
    /// A flag to enable confidential compute mode on boot disk
    #[builder(into, default)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: Box<Option<bool>>,
    /// The image from which this disk was initialised.
    #[builder(into, default)]
    #[serde(rename = "image")]
    pub r#image: Box<Option<String>>,
    /// A set of key/value label pairs assigned to the disk.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Indicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle.
    #[builder(into, default)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: Box<Option<i32>>,
    /// Indicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle.
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
    /// The size of the image in gigabytes.
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<i32>>,
    /// The URL of the storage pool in which the new disk is created
    #[builder(into, default)]
    #[serde(rename = "storagePool")]
    pub r#storage_pool: Box<Option<String>>,
    /// The Google Compute Engine disk type. Such as pd-standard, pd-ssd or pd-balanced.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
