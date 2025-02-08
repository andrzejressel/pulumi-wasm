#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigOrgConfig {
    /// The data to scan folder org or project
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigOrgConfigLocation>>,
    /// The project that will run the scan. The DLP service account that exists within this project must have access to all resources that are profiled, and the cloud DLP API must be enabled.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
}
