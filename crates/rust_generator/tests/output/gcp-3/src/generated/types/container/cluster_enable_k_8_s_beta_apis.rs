#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterEnableK8SBetaApis {
    /// Enabled Kubernetes Beta APIs.
    #[builder(into)]
    #[serde(rename = "enabledApis")]
    pub r#enabled_apis: Box<Vec<String>>,
}
