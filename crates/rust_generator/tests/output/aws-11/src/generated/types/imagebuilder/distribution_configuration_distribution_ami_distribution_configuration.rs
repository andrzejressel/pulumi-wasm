#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionConfigurationDistributionAmiDistributionConfiguration {
    /// Key-value map of tags to apply to the distributed AMI.
    #[builder(into, default)]
    #[serde(rename = "amiTags")]
    pub r#ami_tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Description to apply to the distributed AMI.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key to encrypt the distributed AMI.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Configuration block of EC2 launch permissions to apply to the distributed AMI. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "launchPermission")]
    pub r#launch_permission: Box<Option<super::super::types::imagebuilder::DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission>>,
    /// Name to apply to the distributed AMI.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Set of AWS Account identifiers to distribute the AMI.
    #[builder(into, default)]
    #[serde(rename = "targetAccountIds")]
    pub r#target_account_ids: Box<Option<Vec<String>>>,
}
