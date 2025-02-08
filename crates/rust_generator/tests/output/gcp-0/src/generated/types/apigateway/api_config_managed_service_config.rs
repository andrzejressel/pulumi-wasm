#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApiConfigManagedServiceConfig {
    /// Base64 encoded content of the file.
    #[builder(into)]
    #[serde(rename = "contents")]
    pub r#contents: Box<String>,
    /// The file path (full or relative path). This is typically the path of the file when it is uploaded.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
