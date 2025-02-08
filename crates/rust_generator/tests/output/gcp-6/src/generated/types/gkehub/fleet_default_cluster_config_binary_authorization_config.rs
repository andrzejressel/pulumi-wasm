#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetDefaultClusterConfigBinaryAuthorizationConfig {
    /// Mode of operation for binauthz policy evaluation.
    /// Possible values are: `DISABLED`, `POLICY_BINDINGS`.
    #[builder(into, default)]
    #[serde(rename = "evaluationMode")]
    pub r#evaluation_mode: Box<Option<String>>,
    /// Binauthz policies that apply to this cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "policyBindings")]
    pub r#policy_bindings: Box<Option<Vec<super::super::types::gkehub::FleetDefaultClusterConfigBinaryAuthorizationConfigPolicyBinding>>>,
}
