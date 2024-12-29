#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSupportedInstanceTypesSupportedInstanceType {
    /// CPU architecture.
    #[builder(into)]
    #[serde(rename = "architecture")]
    pub r#architecture: Box<String>,
    /// Indicates whether the instance type supports Amazon EBS optimization.
    #[builder(into)]
    #[serde(rename = "ebsOptimizedAvailable")]
    pub r#ebs_optimized_available: Box<bool>,
    /// Indicates whether the instance type uses Amazon EBS optimization by default.
    #[builder(into)]
    #[serde(rename = "ebsOptimizedByDefault")]
    pub r#ebs_optimized_by_default: Box<bool>,
    /// Indicates whether the instance type only supports Amazon EBS.
    #[builder(into)]
    #[serde(rename = "ebsStorageOnly")]
    pub r#ebs_storage_only: Box<bool>,
    /// The Amazon EC2 family and generation for the instance type.
    #[builder(into)]
    #[serde(rename = "instanceFamilyId")]
    pub r#instance_family_id: Box<String>,
    /// Indicates whether the instance type only supports 64-bit architecture.
    #[builder(into)]
    #[serde(rename = "is64BitsOnly")]
    pub r#is_64_bits_only: Box<bool>,
    /// Memory that is available to Amazon EMR from the instance type.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Box<f64>,
    /// Number of disks for the instance type.
    #[builder(into)]
    #[serde(rename = "numberOfDisks")]
    pub r#number_of_disks: Box<i32>,
    /// Storage capacity of the instance type.
    #[builder(into)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: Box<i32>,
    /// Amazon EC2 instance type. For example, `m5.xlarge`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The number of vCPUs available for the instance type.
    #[builder(into)]
    #[serde(rename = "vcpu")]
    pub r#vcpu: Box<i32>,
}
