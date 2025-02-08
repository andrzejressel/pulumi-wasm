#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDistributionConfigurationDistributionAmiDistributionConfiguration {
    /// Key-value map of tags to apply to distributed AMI.
    #[builder(into)]
    #[serde(rename = "amiTags")]
    pub r#ami_tags: Box<std::collections::HashMap<String, String>>,
    /// Description of the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// ARN of Key Management Service (KMS) Key to encrypt AMI.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<String>,
    /// Nested list of EC2 launch permissions.
    #[builder(into)]
    #[serde(rename = "launchPermissions")]
    pub r#launch_permissions: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission>>,
    /// Name of the distribution configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Set of target AWS Account identifiers.
    #[builder(into)]
    #[serde(rename = "targetAccountIds")]
    pub r#target_account_ids: Box<Vec<String>>,
}
