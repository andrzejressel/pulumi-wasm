#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualClusterContainerProvider {
    /// The name of the container provider that is running your EMR Containers cluster
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Nested list containing information about the configuration of the container provider
    #[builder(into)]
    #[serde(rename = "info")]
    pub r#info: Box<super::super::types::emrcontainers::VirtualClusterContainerProviderInfo>,
    /// The type of the container provider
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
