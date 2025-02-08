#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplate {
    /// A `container` block as defined below.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Vec<super::super::types::containerapp::JobTemplateContainer>>,
    /// A `init_container` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "initContainers")]
    pub r#init_containers: Box<Option<Vec<super::super::types::containerapp::JobTemplateInitContainer>>>,
    /// A `volume` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Option<Vec<super::super::types::containerapp::JobTemplateVolume>>>,
}
