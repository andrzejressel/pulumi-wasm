#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateTemplateVolumeSecretItem {
    /// Integer octal mode bits to use on this file, must be a value between 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be used.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<i32>>,
    /// The relative path of the secret in the container.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
