#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessApplicationSaasAppCustomClaim {
    /// The name of the attribute as provided to the SaaS app.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// True if the attribute must be always present.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    /// The scope of the claim.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<crate::types::AccessApplicationSaasAppCustomClaimSource>,
}
