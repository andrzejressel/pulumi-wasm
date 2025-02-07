#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSigningJobSourceS3 {
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
