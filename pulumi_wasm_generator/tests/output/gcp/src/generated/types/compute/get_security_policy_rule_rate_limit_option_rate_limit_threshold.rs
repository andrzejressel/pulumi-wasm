#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecurityPolicyRuleRateLimitOptionRateLimitThreshold {
    /// Number of HTTP(S) requests for calculating the threshold.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// Interval over which the threshold is computed.
    #[builder(into)]
    #[serde(rename = "intervalSec")]
    pub r#interval_sec: Box<i32>,
}
