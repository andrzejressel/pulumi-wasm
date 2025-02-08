#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigMonitoring {
    /// Specifies the list of backends Policy Controller will export to. An empty list would effectively disable metrics export.
    /// Each value may be one of: `MONITORING_BACKEND_UNSPECIFIED`, `PROMETHEUS`, `CLOUD_MONITORING`.
    #[builder(into, default)]
    #[serde(rename = "backends")]
    pub r#backends: Box<Option<Vec<String>>>,
}
