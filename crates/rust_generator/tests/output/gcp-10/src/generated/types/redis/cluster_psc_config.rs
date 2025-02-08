#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterPscConfig {
    /// Required. The consumer network where the network address of
    /// the discovery endpoint will be reserved, in the form of
    /// projects/{network_project_id_or_number}/global/networks/{network_id}.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
}
