#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct SpectrumApplicationDns {
    /// The name of the DNS record associated with the application.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of DNS record associated with the application.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
