#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetColumnLevelPermissionRule {
    /// An array of column names.
    #[builder(into, default)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Box<Option<Vec<String>>>,
    /// An array of ARNs for Amazon QuickSight users or groups.
    #[builder(into, default)]
    #[serde(rename = "principals")]
    pub r#principals: Box<Option<Vec<String>>>,
}