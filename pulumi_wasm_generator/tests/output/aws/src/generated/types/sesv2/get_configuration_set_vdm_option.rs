#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationSetVdmOption {
    /// Specifies additional settings for your VDM configuration as applicable to the Dashboard.
    #[builder(into)]
    #[serde(rename = "dashboardOptions")]
    pub r#dashboard_options: Box<Vec<super::super::types::sesv2::GetConfigurationSetVdmOptionDashboardOption>>,
    /// Specifies additional settings for your VDM configuration as applicable to the Guardian.
    #[builder(into)]
    #[serde(rename = "guardianOptions")]
    pub r#guardian_options: Box<Vec<super::super::types::sesv2::GetConfigurationSetVdmOptionGuardianOption>>,
}