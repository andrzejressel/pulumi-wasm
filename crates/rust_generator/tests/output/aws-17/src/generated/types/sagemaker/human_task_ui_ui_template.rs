#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HumanTaskUiUiTemplate {
    /// The content of the Liquid template for the worker user interface.
    #[builder(into, default)]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// The SHA-256 digest of the contents of the template.
    #[builder(into, default)]
    #[serde(rename = "contentSha256")]
    pub r#content_sha_256: Box<Option<String>>,
    /// The URL for the user interface template.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}
