#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessApplicationSaasAppCustomAttributeSource {
    /// The name of the footer link.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
