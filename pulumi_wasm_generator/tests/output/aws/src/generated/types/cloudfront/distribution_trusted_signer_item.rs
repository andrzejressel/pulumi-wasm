#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionTrustedSignerItem {
    /// AWS account ID or `self`
    #[builder(into, default)]
    #[serde(rename = "awsAccountNumber")]
    pub r#aws_account_number: Box<Option<String>>,
    /// Set of active CloudFront key pairs associated with the signer account
    #[builder(into, default)]
    #[serde(rename = "keyPairIds")]
    pub r#key_pair_ids: Box<Option<Vec<String>>>,
}