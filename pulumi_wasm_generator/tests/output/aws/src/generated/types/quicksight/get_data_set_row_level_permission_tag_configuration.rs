#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetRowLevelPermissionTagConfiguration {
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    #[builder(into)]
    #[serde(rename = "tagRules")]
    pub r#tag_rules: Box<Vec<super::super::types::quicksight::GetDataSetRowLevelPermissionTagConfigurationTagRule>>,
}