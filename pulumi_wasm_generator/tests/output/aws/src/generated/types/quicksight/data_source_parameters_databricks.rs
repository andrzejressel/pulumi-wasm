#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceParametersDatabricks {
    /// The host name of the Databricks data source.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The port for the Databricks data source.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The HTTP path of the Databricks data source.
    #[builder(into)]
    #[serde(rename = "sqlEndpointPath")]
    pub r#sql_endpoint_path: Box<String>,
}