#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualClusterContainerProvider {
    /// The name of the container provider that is running your EMR Containers cluster
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Nested list containing information about the configuration of the container provider
    #[builder(into)]
    #[serde(rename = "infos")]
    pub r#infos: Box<Vec<super::super::types::emrcontainers::GetVirtualClusterContainerProviderInfo>>,
    /// The type of the container provider
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
