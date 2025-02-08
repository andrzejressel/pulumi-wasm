#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly {
    /// The access token aud claim values that you want to accept in your policy store.
    #[builder(into, default)]
    #[serde(rename = "audiences")]
    pub r#audiences: Box<Option<Vec<String>>>,
    /// The claim that determines the principal in OIDC access tokens.
    #[builder(into, default)]
    #[serde(rename = "principalIdClaim")]
    pub r#principal_id_claim: Box<Option<String>>,
}
