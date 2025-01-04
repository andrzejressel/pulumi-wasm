#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationSelector {
    /// Contains attributes about a target.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Box<Vec<super::super::types::clouddeploy::AutomationSelectorTarget>>,
}
