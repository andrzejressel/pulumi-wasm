#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterControlPlaneRemote {
    /// Name of the Google Distributed Cloud Edge zones where this node pool
    /// will be created. For example: `us-central1-edge-customer-a`.
    #[builder(into, default)]
    #[serde(rename = "nodeLocation")]
    pub r#node_location: Box<Option<String>>,
}
