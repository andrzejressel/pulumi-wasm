#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalAdminClusterValidationCheckStatus {
    /// (Output)
    /// Individual checks which failed as part of the Preflight check execution.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "results")]
    pub r#results: Box<Option<Vec<super::super::types::gkeonprem::BareMetalAdminClusterValidationCheckStatusResult>>>,
}
