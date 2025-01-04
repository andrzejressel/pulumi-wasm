#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterControlPlaneLocal {
    /// Only machines matching this filter will be allowed to host control
    /// plane nodes. The filtering language accepts strings like "name=<name>",
    /// and is documented here: [AIP-160](https://google.aip.dev/160).
    #[builder(into, default)]
    #[serde(rename = "machineFilter")]
    pub r#machine_filter: Box<Option<String>>,
    /// The number of nodes to serve as replicas of the Control Plane.
    /// Only 1 and 3 are supported.
    #[builder(into, default)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<Option<i32>>,
    /// Name of the Google Distributed Cloud Edge zones where this node pool
    /// will be created. For example: `us-central1-edge-customer-a`.
    #[builder(into, default)]
    #[serde(rename = "nodeLocation")]
    pub r#node_location: Box<Option<String>>,
    /// Policy configuration about how user applications are deployed.
    /// Possible values are: `SHARED_DEPLOYMENT_POLICY_UNSPECIFIED`, `ALLOWED`, `DISALLOWED`.
    #[builder(into, default)]
    #[serde(rename = "sharedDeploymentPolicy")]
    pub r#shared_deployment_policy: Box<Option<String>>,
}
