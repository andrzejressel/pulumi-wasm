#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterControlPlane {
    /// Local control plane configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "local")]
    pub r#local: Box<Option<super::super::types::edgecontainer::ClusterControlPlaneLocal>>,
    /// Remote control plane configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "remote")]
    pub r#remote: Box<Option<super::super::types::edgecontainer::ClusterControlPlaneRemote>>,
}
