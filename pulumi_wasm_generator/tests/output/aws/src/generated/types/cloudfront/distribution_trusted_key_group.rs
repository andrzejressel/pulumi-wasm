#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionTrustedKeyGroup {
    /// `true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// List of nested attributes for each trusted signer
    #[builder(into, default)]
    #[serde(rename = "items")]
    pub r#items: Box<Option<Vec<super::super::types::cloudfront::DistributionTrustedKeyGroupItem>>>,
}
