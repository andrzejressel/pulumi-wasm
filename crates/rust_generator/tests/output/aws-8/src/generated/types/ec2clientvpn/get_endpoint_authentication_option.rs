#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointAuthenticationOption {
    #[builder(into)]
    #[serde(rename = "activeDirectoryId")]
    pub r#active_directory_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "rootCertificateChainArn")]
    pub r#root_certificate_chain_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "samlProviderArn")]
    pub r#saml_provider_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "selfServiceSamlProviderArn")]
    pub r#self_service_saml_provider_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
