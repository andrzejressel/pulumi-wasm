#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAlertRuleTemplateSecurityIncidentTemplate {
    /// The description of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The Microsoft Security Service from where the alert will be generated.
    #[builder(into)]
    #[serde(rename = "productFilter")]
    pub r#product_filter: Box<String>,
}
