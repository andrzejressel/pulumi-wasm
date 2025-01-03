#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UsagePlanQuotaSettings {
    /// Maximum number of requests that can be made in a given time period.
    #[builder(into)]
    #[serde(rename = "limit")]
    pub r#limit: Box<i32>,
    /// Number of requests subtracted from the given limit in the initial time period.
    #[builder(into, default)]
    #[serde(rename = "offset")]
    pub r#offset: Box<Option<i32>>,
    /// Time period in which the limit applies. Valid values are "DAY", "WEEK" or "MONTH".
    #[builder(into)]
    #[serde(rename = "period")]
    pub r#period: Box<String>,
}
