#[derive(serde::Serialize)]
pub struct ListItemRedirect {
    /// Whether the redirect also matches subdomains of the source url.
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    /// Whether the redirect target url should keep the query string of the request's url.
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Box<Option<bool>>,
    /// Whether the redirect target url should keep the query string of the request's url.
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    /// The source url of the redirect.
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
    /// The status code to be used when redirecting a request.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Whether the redirect also matches subpaths of the source url.
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Box<Option<bool>>,
    /// The target url of the redirect.
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<String>,
}
