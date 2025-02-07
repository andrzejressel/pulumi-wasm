#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalAdminClusterValidationCheck {
    /// (Output)
    /// Options used for the validation check.
    #[builder(into, default)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<String>>,
    /// (Output)
    /// The scenario when the preflight checks were run..
    #[builder(into, default)]
    #[serde(rename = "scenario")]
    pub r#scenario: Box<Option<String>>,
    /// (Output)
    /// Specifies the detailed validation check status
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<super::super::types::gkeonprem::BareMetalAdminClusterValidationCheckStatus>>>,
}
