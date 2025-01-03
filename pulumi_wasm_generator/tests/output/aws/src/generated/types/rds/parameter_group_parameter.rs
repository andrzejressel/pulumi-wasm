#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ParameterGroupParameter {
    /// "immediate" (default), or "pending-reboot". Some
    /// engines can't apply some parameters without a reboot, and you will need to
    /// specify "pending-reboot" here.
    #[builder(into, default)]
    #[serde(rename = "applyMethod")]
    pub r#apply_method: Box<Option<String>>,
    /// The name of the DB parameter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the DB parameter.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
