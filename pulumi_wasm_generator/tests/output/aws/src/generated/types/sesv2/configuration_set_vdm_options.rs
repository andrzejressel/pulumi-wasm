#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetVdmOptions {
    /// Specifies additional settings for your VDM configuration as applicable to the Dashboard. See `dashboard_options` Block for details.
    #[builder(into, default)]
    #[serde(rename = "dashboardOptions")]
    pub r#dashboard_options: Box<Option<super::super::types::sesv2::ConfigurationSetVdmOptionsDashboardOptions>>,
    /// Specifies additional settings for your VDM configuration as applicable to the Guardian. See `guardian_options` Block for details.
    #[builder(into, default)]
    #[serde(rename = "guardianOptions")]
    pub r#guardian_options: Box<Option<super::super::types::sesv2::ConfigurationSetVdmOptionsGuardianOptions>>,
}