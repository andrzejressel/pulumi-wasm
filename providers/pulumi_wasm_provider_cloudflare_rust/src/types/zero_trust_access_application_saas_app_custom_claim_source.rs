#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessApplicationSaasAppCustomClaimSource {
    /// The name of the attribute as provided by the IDP.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A mapping from IdP ID to claim name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "nameByIdp")]
    pub r#name_by_idp: Box<Option<std::collections::HashMap<String, String>>>,
}
