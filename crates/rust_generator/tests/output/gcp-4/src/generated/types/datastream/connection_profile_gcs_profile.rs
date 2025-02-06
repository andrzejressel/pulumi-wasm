#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileGcsProfile {
    /// The Cloud Storage bucket name.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// The root path inside the Cloud Storage bucket.
    #[builder(into, default)]
    #[serde(rename = "rootPath")]
    pub r#root_path: Box<Option<String>>,
}
