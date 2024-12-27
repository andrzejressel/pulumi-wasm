#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTaskSpecNetworksAdvanced {
    /// The network aliases of the container in the specific network.
    #[builder(into, default)]
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    /// An array of driver options for the network, e.g. `opts1=value`
    #[builder(into, default)]
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Box<Option<Vec<String>>>,
    /// The name/id of the network.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
