#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceTemplateSpecContainerStartupProbeHttpGetHttpHeader {
    /// The name of the Cloud Run Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The header field value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
