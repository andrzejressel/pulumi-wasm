#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterPscConfig {
    /// Create an instance that allows connections from Private Service Connect endpoints to the instance.
    #[builder(into, default)]
    #[serde(rename = "pscEnabled")]
    pub r#psc_enabled: Box<Option<bool>>,
}
