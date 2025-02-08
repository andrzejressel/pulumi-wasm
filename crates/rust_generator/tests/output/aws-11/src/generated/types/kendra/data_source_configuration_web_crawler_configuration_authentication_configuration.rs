#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceConfigurationWebCrawlerConfigurationAuthenticationConfiguration {
    /// The list of configuration information that's required to connect to and crawl a website host using basic authentication credentials. The list includes the name and port number of the website host. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "basicAuthentications")]
    pub r#basic_authentications: Box<Option<Vec<super::super::types::kendra::DataSourceConfigurationWebCrawlerConfigurationAuthenticationConfigurationBasicAuthentication>>>,
}
