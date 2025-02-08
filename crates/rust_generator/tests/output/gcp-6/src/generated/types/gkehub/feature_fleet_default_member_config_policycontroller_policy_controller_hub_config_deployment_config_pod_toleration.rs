#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration {
    /// Matches a taint effect.
    #[builder(into, default)]
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    /// Matches a taint key (not necessarily unique).
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Matches a taint operator.
    #[builder(into, default)]
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
    /// Matches a taint value.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
