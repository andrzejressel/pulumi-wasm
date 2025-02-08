#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataQualityJobDefinitionJobResourcesClusterConfig {
    /// The number of ML compute instances to use in the model monitoring job. For distributed processing jobs, specify a value greater than 1.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Box<i32>,
    /// The ML compute instance type for the processing job.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance(s) that run the model monitoring job.
    #[builder(into, default)]
    #[serde(rename = "volumeKmsKeyId")]
    pub r#volume_kms_key_id: Box<Option<String>>,
    /// The size of the ML storage volume, in gigabytes, that you want to provision. You must specify sufficient ML storage for your scenario.
    #[builder(into)]
    #[serde(rename = "volumeSizeInGb")]
    pub r#volume_size_in_gb: Box<i32>,
}
