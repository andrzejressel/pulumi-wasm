#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountCertificate {
    /// ID of certificate for TLS interception.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
