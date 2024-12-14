#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountFips {
    /// Only allow FIPS-compliant TLS configuration.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<bool>>,
}
