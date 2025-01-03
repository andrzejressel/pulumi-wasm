#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTemplatesTemplate {
    /// Indicates whether the quota is global.
    #[builder(into)]
    #[serde(rename = "globalQuota")]
    pub r#global_quota: Box<bool>,
    /// Quota identifier.
    #[builder(into)]
    #[serde(rename = "quotaCode")]
    pub r#quota_code: Box<String>,
    /// Quota name.
    #[builder(into)]
    #[serde(rename = "quotaName")]
    pub r#quota_name: Box<String>,
    /// AWS Region to which the quota increases apply.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// (Required) Service identifier.
    #[builder(into)]
    #[serde(rename = "serviceCode")]
    pub r#service_code: Box<String>,
    /// Service name.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<String>,
    /// Unit of measurement.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
    /// (Required) The new, increased value for the quota.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<f64>,
}
