#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetRowLevelPermissionTagConfigurationTagRule {
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "matchAllValue")]
    pub r#match_all_value: Box<String>,
    #[builder(into)]
    #[serde(rename = "tagKey")]
    pub r#tag_key: Box<String>,
    #[builder(into)]
    #[serde(rename = "tagMultiValueDelimiter")]
    pub r#tag_multi_value_delimiter: Box<String>,
}