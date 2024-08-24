#[derive(serde::Serialize)]
pub struct ServiceTaskSpecNetworksAdvanced {
    /// The network aliases of the container in the specific network.
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    /// An array of driver options for the network, e.g. `opts1=value`
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Box<Option<Vec<String>>>,
    /// The name/id of the network.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
