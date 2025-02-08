#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomLogSourceConfiguration {
    /// The configuration for the Glue Crawler for the third-party custom source.
    #[builder(into, default)]
    #[serde(rename = "crawlerConfiguration")]
    pub r#crawler_configuration: Box<Option<super::super::types::securitylake::CustomLogSourceConfigurationCrawlerConfiguration>>,
    /// The identity of the log provider for the third-party custom source.
    #[builder(into, default)]
    #[serde(rename = "providerIdentity")]
    pub r#provider_identity: Box<Option<super::super::types::securitylake::CustomLogSourceConfigurationProviderIdentity>>,
}
