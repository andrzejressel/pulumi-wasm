#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StateMachineEncryptionConfiguration {
    /// Maximum duration for which Step Functions will reuse data keys. When the period expires, Step Functions will call GenerateDataKey. This setting only applies to customer managed KMS key and does not apply when `type` is `AWS_OWNED_KEY`.
    #[builder(into, default)]
    #[serde(rename = "kmsDataKeyReusePeriodSeconds")]
    pub r#kms_data_key_reuse_period_seconds: Box<Option<i32>>,
    /// The alias, alias ARN, key ID, or key ARN of the symmetric encryption KMS key that encrypts the data key. To specify a KMS key in a different AWS account, the customer must use the key ARN or alias ARN. For more information regarding kms_key_id, see [KeyId](https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters) in the KMS documentation.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// The encryption option specified for the state machine. Valid values: `AWS_OWNED_KEY`, `CUSTOMER_MANAGED_KMS_KEY`
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
