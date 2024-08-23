#[derive(serde::Serialize)]
pub struct ServiceTaskSpecNetworksAdvanced {
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Box<Option<Vec<String>>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
