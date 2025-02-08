#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetIdentityPoolCognitoIdentityProvider {
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "providerName")]
    pub r#provider_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "serverSideTokenCheck")]
    pub r#server_side_token_check: Box<bool>,
}
