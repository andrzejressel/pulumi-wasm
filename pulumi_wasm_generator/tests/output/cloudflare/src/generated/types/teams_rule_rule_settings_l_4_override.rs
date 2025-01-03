#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TeamsRuleRuleSettingsL4Override {
    /// Override IP to forward traffic to.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    /// Override Port to forward traffic to.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
