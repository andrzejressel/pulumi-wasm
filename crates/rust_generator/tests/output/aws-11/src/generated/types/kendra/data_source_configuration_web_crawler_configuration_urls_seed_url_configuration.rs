#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration {
    /// The list of seed or starting point URLs of the websites you want to crawl. The list can include a maximum of `100` seed URLs. Array Members: Minimum number of `0` items. Maximum number of `100` items. Length Constraints: Minimum length of `1`. Maximum length of `2048`.
    #[builder(into)]
    #[serde(rename = "seedUrls")]
    pub r#seed_urls: Box<Vec<String>>,
    /// The default mode is set to `HOST_ONLY`. You can choose one of the following modes:
    /// * `HOST_ONLY` – crawl only the website host names. For example, if the seed URL is `"abc.example.com"`, then only URLs with host name `"abc.example.com"` are crawled.
    /// * `SUBDOMAINS` – crawl the website host names with subdomains. For example, if the seed URL is `"abc.example.com"`, then `"a.abc.example.com"` and `"b.abc.example.com"` are also crawled.
    /// * `EVERYTHING` – crawl the website host names with subdomains and other domains that the webpages link to.
    #[builder(into, default)]
    #[serde(rename = "webCrawlerMode")]
    pub r#web_crawler_mode: Box<Option<String>>,
}
