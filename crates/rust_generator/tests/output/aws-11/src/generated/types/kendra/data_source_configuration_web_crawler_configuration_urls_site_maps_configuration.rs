#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceConfigurationWebCrawlerConfigurationUrlsSiteMapsConfiguration {
    /// The list of sitemap URLs of the websites you want to crawl. The list can include a maximum of `3` sitemap URLs.
    #[builder(into)]
    #[serde(rename = "siteMaps")]
    pub r#site_maps: Box<Vec<String>>,
}
