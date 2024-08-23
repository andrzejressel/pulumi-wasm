#[derive(serde::Serialize)]
pub struct ListItemRedirect {
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Box<Option<bool>>,
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<bool>>,
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Box<Option<bool>>,
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<String>,
}
