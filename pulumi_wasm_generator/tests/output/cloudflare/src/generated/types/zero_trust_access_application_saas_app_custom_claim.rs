#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessApplicationSaasAppCustomClaim {
    /// The name of the attribute as provided to the SaaS app.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// True if the attribute must be always present.
    #[builder(into, default)]
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    /// The scope of the claim.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::types::ZeroTrustAccessApplicationSaasAppCustomClaimSource>,
}
