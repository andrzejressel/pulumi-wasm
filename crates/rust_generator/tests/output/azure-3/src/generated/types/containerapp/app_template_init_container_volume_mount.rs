#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppTemplateInitContainerVolumeMount {
    /// The name of the Volume to be mounted in the container.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The path in the container at which to mount this volume.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
