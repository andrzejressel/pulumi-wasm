#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureFleetDefaultMemberConfigPolicycontroller {
    /// Configuration of Policy Controller
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyControllerHubConfig")]
    pub r#policy_controller_hub_config: Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfig>,
    /// Configures the version of Policy Controller
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
