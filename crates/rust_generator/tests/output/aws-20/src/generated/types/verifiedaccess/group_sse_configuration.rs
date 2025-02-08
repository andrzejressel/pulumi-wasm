#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GroupSseConfiguration {
    /// Boolean flag to indicate that the CMK should be used.
    #[builder(into, default)]
    #[serde(rename = "customerManagedKeyEnabled")]
    pub r#customer_managed_key_enabled: Box<Option<bool>>,
    /// ARN of the KMS key to use.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
}
