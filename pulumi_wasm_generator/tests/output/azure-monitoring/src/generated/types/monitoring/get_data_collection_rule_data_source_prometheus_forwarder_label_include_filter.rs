#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataCollectionRuleDataSourcePrometheusForwarderLabelIncludeFilter {
    /// The label of the filter. This label should be unique across all `label_include_fileter` block. Possible value is `microsoft_metrics_include_label`.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// The value of the filter.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}