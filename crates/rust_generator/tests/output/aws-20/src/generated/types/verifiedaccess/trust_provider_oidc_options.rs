#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TrustProviderOidcOptions {
    #[builder(into, default)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "userInfoEndpoint")]
    pub r#user_info_endpoint: Box<Option<String>>,
}
