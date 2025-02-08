#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentVmImage {
    /// Use this VM image family to find the image; the newest image in this family will be used.
    #[builder(into, default)]
    #[serde(rename = "imageFamily")]
    pub r#image_family: Box<Option<String>>,
    /// Use VM image name to find the image.
    #[builder(into, default)]
    #[serde(rename = "imageName")]
    pub r#image_name: Box<Option<String>>,
    /// The name of the Google Cloud project that this VM image belongs to.
    /// Format: projects/{project_id}
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
}
