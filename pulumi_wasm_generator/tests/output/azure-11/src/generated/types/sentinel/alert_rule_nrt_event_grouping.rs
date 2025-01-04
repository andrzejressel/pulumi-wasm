#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleNrtEventGrouping {
    /// The aggregation type of grouping the events. Possible values are `AlertPerResult` and `SingleAlert`.
    #[builder(into)]
    #[serde(rename = "aggregationMethod")]
    pub r#aggregation_method: Box<String>,
}
