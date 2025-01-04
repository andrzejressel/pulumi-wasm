#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxWebAppSiteConfigAutoHealSetting {
    /// A `action` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::appservice::LinuxWebAppSiteConfigAutoHealSettingAction>>,
    /// A `trigger` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<Option<super::super::types::appservice::LinuxWebAppSiteConfigAutoHealSettingTrigger>>,
}
