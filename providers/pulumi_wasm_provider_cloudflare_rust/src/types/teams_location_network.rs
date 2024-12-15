#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsLocationNetwork {
    /// The ID of this resource.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// CIDR notation representation of the network IP.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
}
