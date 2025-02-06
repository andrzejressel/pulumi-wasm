#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsWebAppSiteConfigAutoHealSettingAction {
    /// The predefined action to be taken to an Auto Heal trigger.
    #[builder(into)]
    #[serde(rename = "actionType")]
    pub r#action_type: Box<String>,
    /// A `custom_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "customActions")]
    pub r#custom_actions: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingActionCustomAction>>,
    /// The minimum amount of time in `hh:mm:ss` the Windows Web App must have been running before the defined action will be run in the event of a trigger.
    #[builder(into)]
    #[serde(rename = "minimumProcessExecutionTime")]
    pub r#minimum_process_execution_time: Box<String>,
}
