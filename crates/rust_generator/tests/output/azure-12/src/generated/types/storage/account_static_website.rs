#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountStaticWebsite {
    /// The absolute path to a custom webpage that should be used when a request is made which does not correspond to an existing file.
    #[builder(into, default)]
    #[serde(rename = "error404Document")]
    pub r#error_404_document: Box<Option<String>>,
    /// The webpage that Azure Storage serves for requests to the root of a website or any subfolder. For example, index.html. The value is case-sensitive.
    #[builder(into, default)]
    #[serde(rename = "indexDocument")]
    pub r#index_document: Box<Option<String>>,
}
