#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSlotSiteConfigAutoHealSettingActionCustomAction {
    /// The executable to run for the `custom_action`.
    #[builder(into)]
    #[serde(rename = "executable")]
    pub r#executable: Box<String>,
    /// The parameters to pass to the specified `executable`.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<String>>,
}