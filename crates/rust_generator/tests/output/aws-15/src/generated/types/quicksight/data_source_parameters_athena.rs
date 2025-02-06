#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceParametersAthena {
    /// The work-group to which to connect.
    #[builder(into, default)]
    #[serde(rename = "workGroup")]
    pub r#work_group: Box<Option<String>>,
}
