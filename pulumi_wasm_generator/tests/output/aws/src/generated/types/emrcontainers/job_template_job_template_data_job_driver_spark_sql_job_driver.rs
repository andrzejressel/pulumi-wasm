#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateJobTemplateDataJobDriverSparkSqlJobDriver {
    /// The SQL file to be executed.
    #[builder(into, default)]
    #[serde(rename = "entryPoint")]
    pub r#entry_point: Box<Option<String>>,
    /// The Spark parameters to be included in the Spark SQL command.
    #[builder(into, default)]
    #[serde(rename = "sparkSqlParameters")]
    pub r#spark_sql_parameters: Box<Option<String>>,
}