#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct SpectrumApplicationOriginDns {
    /// Fully qualified domain name of the origin.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
