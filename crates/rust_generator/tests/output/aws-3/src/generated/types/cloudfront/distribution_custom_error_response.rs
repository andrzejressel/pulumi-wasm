#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionCustomErrorResponse {
    /// Minimum amount of time you want HTTP error codes to stay in CloudFront caches before CloudFront queries your origin to see whether the object has been updated.
    #[builder(into, default)]
    #[serde(rename = "errorCachingMinTtl")]
    pub r#error_caching_min_ttl: Box<Option<i32>>,
    /// 4xx or 5xx HTTP status code that you want to customize.
    #[builder(into)]
    #[serde(rename = "errorCode")]
    pub r#error_code: Box<i32>,
    /// HTTP status code that you want CloudFront to return with the custom error page to the viewer.
    #[builder(into, default)]
    #[serde(rename = "responseCode")]
    pub r#response_code: Box<Option<i32>>,
    /// Path of the custom error page (for example, `/custom_404.html`).
    #[builder(into, default)]
    #[serde(rename = "responsePagePath")]
    pub r#response_page_path: Box<Option<String>>,
}
