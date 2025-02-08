#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetWindowsWebAppSiteConfigAutoHealSettingActionCustomAction {
    /// The command run when this `auto_heal` action is triggered.
    #[builder(into)]
    #[serde(rename = "executable")]
    pub r#executable: Box<String>,
    /// The parameters passed to the `executable`.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<String>,
}
