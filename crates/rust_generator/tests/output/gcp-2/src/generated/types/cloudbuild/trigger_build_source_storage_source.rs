#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerBuildSourceStorageSource {
    /// Google Cloud Storage bucket containing the source.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Google Cloud Storage generation for the object.
    /// If the generation is omitted, the latest generation will be used
    #[builder(into, default)]
    #[serde(rename = "generation")]
    pub r#generation: Box<Option<String>>,
    /// Google Cloud Storage object containing the source.
    /// This object must be a gzipped archive file (.tar.gz) containing source to build.
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
}
