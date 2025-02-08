#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateSpecContainerVolumeMount {
    /// Path within the container at which the volume should be mounted.  Must
    /// not contain ':'.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<String>,
    /// This must match the Name of a Volume.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
