#[derive(serde::Serialize)]
pub struct AccessOrganizationCustomPage {
    #[serde(rename = "forbidden")]
    pub r#forbidden: Box<Option<String>>,
    #[serde(rename = "identityDenied")]
    pub r#identity_denied: Box<Option<String>>,
}
