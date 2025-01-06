#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkInsightsAnalysisReturnPathComponentSecurityGroupRulePortRange {
    #[builder(into, default)]
    #[serde(rename = "from")]
    pub r#from: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "to")]
    pub r#to: Box<Option<i32>>,
}
