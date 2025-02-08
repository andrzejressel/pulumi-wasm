#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBucketWebsite {
    /// Behaves as the bucket's directory index where missing objects are treated as potential directories.
    #[builder(into)]
    #[serde(rename = "mainPageSuffix")]
    pub r#main_page_suffix: Box<String>,
    /// The custom object to return when a requested resource is not found.
    #[builder(into)]
    #[serde(rename = "notFoundPage")]
    pub r#not_found_page: Box<String>,
}
