#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterEncryptionInfo {
    /// You may specify a KMS key short ID or ARN (it will always output an ARN) to use for encrypting your data at rest.  If no key is specified, an AWS managed KMS ('aws/msk' managed service) key will be used for encrypting the data at rest.
    #[builder(into, default)]
    #[serde(rename = "encryptionAtRestKmsKeyArn")]
    pub r#encryption_at_rest_kms_key_arn: Box<Option<String>>,
    /// Configuration block to specify encryption in transit. See below.
    #[builder(into, default)]
    #[serde(rename = "encryptionInTransit")]
    pub r#encryption_in_transit: Box<Option<super::super::types::msk::ClusterEncryptionInfoEncryptionInTransit>>,
}
