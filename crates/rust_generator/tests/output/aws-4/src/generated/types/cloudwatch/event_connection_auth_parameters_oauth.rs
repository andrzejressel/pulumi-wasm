#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EventConnectionAuthParametersOauth {
    /// The URL to the authorization endpoint.
    #[builder(into)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: Box<String>,
    /// Contains the client parameters for OAuth authorization. Contains the following two parameters.
    #[builder(into, default)]
    #[serde(rename = "clientParameters")]
    pub r#client_parameters: Box<Option<super::super::types::cloudwatch::EventConnectionAuthParametersOauthClientParameters>>,
    /// A password for the authorization. Created and stored in AWS Secrets Manager.
    #[builder(into)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Box<String>,
    /// OAuth Http Parameters are additional credentials used to sign the request to the authorization endpoint to exchange the OAuth Client information for an access token. Secret values are stored and managed by AWS Secrets Manager. A maximum of 1 are allowed. Documented below.
    #[builder(into)]
    #[serde(rename = "oauthHttpParameters")]
    pub r#oauth_http_parameters: Box<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParameters>,
}
