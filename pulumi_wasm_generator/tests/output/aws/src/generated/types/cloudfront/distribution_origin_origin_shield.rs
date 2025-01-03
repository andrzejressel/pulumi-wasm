#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionOriginOriginShield {
    /// `true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// AWS Region for Origin Shield. To specify a region, use the region code, not the region name. For example, specify the US East (Ohio) region as `us-east-2`.
    #[builder(into, default)]
    #[serde(rename = "originShieldRegion")]
    pub r#origin_shield_region: Box<Option<String>>,
}
