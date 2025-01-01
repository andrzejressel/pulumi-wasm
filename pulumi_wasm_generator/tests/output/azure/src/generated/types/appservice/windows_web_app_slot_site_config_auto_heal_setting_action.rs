#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSlotSiteConfigAutoHealSettingAction {
    /// Predefined action to be taken to an Auto Heal trigger. Possible values are `CustomAction`, `LogEvent` and `Recycle`.
    #[builder(into)]
    #[serde(rename = "actionType")]
    pub r#action_type: Box<String>,
    /// A `custom_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "customAction")]
    pub r#custom_action: Box<Option<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingActionCustomAction>>,
    /// The minimum amount of time in `hh:mm:ss` the Windows Web App Slot must have been running before the defined action will be run in the event of a trigger.
    #[builder(into, default)]
    #[serde(rename = "minimumProcessExecutionTime")]
    pub r#minimum_process_execution_time: Box<Option<String>>,
}
