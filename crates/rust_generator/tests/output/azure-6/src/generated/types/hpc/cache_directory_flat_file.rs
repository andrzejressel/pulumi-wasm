#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CacheDirectoryFlatFile {
    /// The URI of the file containing group information (`/etc/group` file format in Unix-like OS).
    #[builder(into)]
    #[serde(rename = "groupFileUri")]
    pub r#group_file_uri: Box<String>,
    /// The URI of the file containing user information (`/etc/passwd` file format in Unix-like OS).
    #[builder(into)]
    #[serde(rename = "passwordFileUri")]
    pub r#password_file_uri: Box<String>,
}
