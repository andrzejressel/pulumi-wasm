#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StackSetAutoDeployment {
    /// Whether or not auto-deployment is enabled.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Whether or not to retain stacks when the account is removed.
    #[builder(into, default)]
    #[serde(rename = "retainStacksOnAccountRemoval")]
    pub r#retain_stacks_on_account_removal: Box<Option<bool>>,
}