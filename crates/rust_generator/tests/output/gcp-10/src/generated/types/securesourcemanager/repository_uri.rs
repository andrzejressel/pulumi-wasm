#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryUri {
    /// (Output)
    /// API is the URI for API access.
    #[builder(into, default)]
    #[serde(rename = "api")]
    pub r#api: Box<Option<String>>,
    /// (Output)
    /// git_https is the git HTTPS URI for git operations.
    #[builder(into, default)]
    #[serde(rename = "gitHttps")]
    pub r#git_https: Box<Option<String>>,
    /// (Output)
    /// HTML is the URI for the user to view the repository in a browser.
    #[builder(into, default)]
    #[serde(rename = "html")]
    pub r#html: Box<Option<String>>,
}
