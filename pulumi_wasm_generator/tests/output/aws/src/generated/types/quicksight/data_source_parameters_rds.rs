#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceParametersRds {
    /// The database to which to connect.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    /// The instance ID to which to connect.
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Box<String>,
}
