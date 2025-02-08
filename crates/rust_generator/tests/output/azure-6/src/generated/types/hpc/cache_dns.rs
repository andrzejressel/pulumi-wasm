#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CacheDns {
    /// The DNS search domain for the HPC Cache.
    #[builder(into, default)]
    #[serde(rename = "searchDomain")]
    pub r#search_domain: Box<Option<String>>,
    /// A list of DNS servers for the HPC Cache. At most three IP(s) are allowed to set.
    #[builder(into)]
    #[serde(rename = "servers")]
    pub r#servers: Box<Vec<String>>,
}
