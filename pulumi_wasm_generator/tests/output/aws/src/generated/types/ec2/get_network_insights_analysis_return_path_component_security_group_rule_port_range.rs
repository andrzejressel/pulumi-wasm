#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkInsightsAnalysisReturnPathComponentSecurityGroupRulePortRange {
    #[builder(into)]
    #[serde(rename = "from")]
    pub r#from: Box<i32>,
    #[builder(into)]
    #[serde(rename = "to")]
    pub r#to: Box<i32>,
}