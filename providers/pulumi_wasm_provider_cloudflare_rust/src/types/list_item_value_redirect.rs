#[derive(serde::Serialize)]
pub struct ListItemValueRedirect {
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<String>>,
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Box<Option<String>>,
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<String>>,
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Box<Option<String>>,
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<String>,
}
