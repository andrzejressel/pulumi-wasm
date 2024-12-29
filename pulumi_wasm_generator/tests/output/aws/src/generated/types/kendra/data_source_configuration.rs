#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceConfiguration {
    /// A block that provides the configuration information to connect to an Amazon S3 bucket as your data source. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<Option<super::super::types::kendra::DataSourceConfigurationS3Configuration>>,
    /// A block that provides the configuration information required for Amazon Kendra Web Crawler. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "webCrawlerConfiguration")]
    pub r#web_crawler_configuration: Box<Option<super::super::types::kendra::DataSourceConfigurationWebCrawlerConfiguration>>,
}
