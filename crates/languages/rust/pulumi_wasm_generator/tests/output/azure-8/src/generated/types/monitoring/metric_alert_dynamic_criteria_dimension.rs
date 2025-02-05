#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetricAlertDynamicCriteriaDimension {
    /// One of the dimension names.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The dimension operator. Possible values are `Include`, `Exclude` and `StartsWith`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// The list of dimension values.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
