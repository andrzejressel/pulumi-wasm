#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsLocationNetwork {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// CIDR notation representation of the network IP.
    #[serde(rename = "network")]
    pub r#network: Box<String>,
}
