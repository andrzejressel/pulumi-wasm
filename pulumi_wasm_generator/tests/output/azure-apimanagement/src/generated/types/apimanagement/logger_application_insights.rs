#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoggerApplicationInsights {
    /// The connection string of Application Insights.
    #[builder(into, default)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<Option<String>>,
    /// The instrumentation key used to push data to Application Insights.
    /// 
    /// > **Note:** Either `connection_string` or `instrumentation_key` have to be specified.
    #[builder(into, default)]
    #[serde(rename = "instrumentationKey")]
    pub r#instrumentation_key: Box<Option<String>>,
}