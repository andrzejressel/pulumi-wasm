#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayPrivateLinkConfiguration {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// One or more `ip_configuration` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Box<Vec<super::super::types::network::GetApplicationGatewayPrivateLinkConfigurationIpConfiguration>>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
