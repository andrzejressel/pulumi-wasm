#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterFleet {
    /// (Output)
    /// The name of the managed Hub Membership resource associated to this cluster.
    /// Membership names are formatted as
    /// `projects/<project-number>/locations/global/membership/<cluster-id>`.
    #[builder(into, default)]
    #[serde(rename = "membership")]
    pub r#membership: Box<Option<String>>,
    /// The name of the Fleet host project where this cluster will be registered.
    /// Project names are formatted as
    /// `projects/<project-number>`.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
}
