#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionAppSlotAuthSettingsGoogle {
    /// The OpenID Connect Client ID for the Google web application.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The client secret associated with the Google web application.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<String>,
    /// The OAuth 2.0 scopes that will be requested as part of Google Sign-In authentication. <https://developers.google.com/identity/sign-in/web/>
    #[builder(into, default)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Box<Option<Vec<String>>>,
}
