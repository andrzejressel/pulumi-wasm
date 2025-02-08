#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WindowsWebAppSlotSiteConfigAutoHealSetting {
    /// A `action` block as defined above.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingAction>,
    /// A `trigger` block as defined below.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTrigger>,
}
