#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessOrganizationCustomPage {
    /// The id of the forbidden page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "forbidden")]
    pub r#forbidden: Box<Option<String>>,
    /// The id of the identity denied page.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "identityDenied")]
    pub r#identity_denied: Box<Option<String>>,
}
