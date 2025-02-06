#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceGceSetupVmImage {
    /// Optional. Use this VM image family to find the image; the newest
    /// image in this family will be used.
    #[builder(into, default)]
    #[serde(rename = "family")]
    pub r#family: Box<Option<String>>,
    /// Optional. Use VM image name to find the image.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The name of the Google Cloud project that this VM image belongs to.
    /// Format: {project_id}
    #[builder(into, default)]
    #[serde(rename = "project")]
    pub r#project: Box<Option<String>>,
}
