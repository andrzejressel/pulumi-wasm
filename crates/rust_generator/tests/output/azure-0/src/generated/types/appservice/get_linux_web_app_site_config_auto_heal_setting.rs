#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxWebAppSiteConfigAutoHealSetting {
    /// A `action` block as defined above.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigAutoHealSettingAction>>,
    /// A `trigger` block as defined below.
    #[builder(into)]
    #[serde(rename = "triggers")]
    pub r#triggers: Box<Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigAutoHealSettingTrigger>>,
}
