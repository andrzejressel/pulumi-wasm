#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CostCategorySplitChargeRule {
    /// Method that's used to define how to split your source costs across your targets. Valid values are `FIXED`, `PROPORTIONAL`, `EVEN`
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Box<String>,
    /// Configuration block for the parameters for a split charge method. This is only required for the `FIXED` method. See below.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::costexplorer::CostCategorySplitChargeRuleParameter>>>,
    /// Cost Category value that you want to split.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
    /// Cost Category values that you want to split costs across. These values can't be used as a source in other split charge rules.
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Box<Vec<String>>,
}