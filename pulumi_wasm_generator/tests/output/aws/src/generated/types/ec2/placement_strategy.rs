#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum PlacementStrategy {
    /// A `spread` placement group places instances on distinct hardware.
    #[serde(rename = "spread")]
    Spread,
    /// A `cluster` placement group is a logical grouping of instances within a single
    /// Availability Zone that benefit from low network latency, high network throughput.
    #[serde(rename = "cluster")]
    Cluster,
}
