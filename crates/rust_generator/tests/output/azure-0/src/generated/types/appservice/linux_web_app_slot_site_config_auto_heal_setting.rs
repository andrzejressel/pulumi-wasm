#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxWebAppSlotSiteConfigAutoHealSetting {
    /// A `action` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingAction>>,
    /// A `trigger` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<Option<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTrigger>>,
}
