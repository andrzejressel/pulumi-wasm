#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecDnsConfig {
    #[serde(rename = "nameservers")]
    pub r#nameservers: Box<Vec<String>>,
    #[serde(rename = "options")]
    pub r#options: Box<Option<Vec<String>>>,
    #[serde(rename = "searches")]
    pub r#searches: Box<Option<Vec<String>>>,
}
