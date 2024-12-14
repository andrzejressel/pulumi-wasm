#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecNetworksAdvanced {
    /// The network aliases of the container in the specific network.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    /// An array of driver options for the network, e.g. `opts1=value`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Box<Option<Vec<String>>>,
    /// The name/id of the network.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
