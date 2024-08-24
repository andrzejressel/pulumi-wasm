#[derive(serde::Serialize)]
pub struct AccessOrganizationCustomPage {
    /// The id of the forbidden page.
    #[serde(rename = "forbidden")]
    pub r#forbidden: Box<Option<String>>,
    /// The id of the identity denied page.
    #[serde(rename = "identityDenied")]
    pub r#identity_denied: Box<Option<String>>,
}
