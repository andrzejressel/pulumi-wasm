#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterLoggingConfig {
    /// The GKE components exposing logs. Supported values include:
    /// `SYSTEM_COMPONENTS`, `APISERVER`, `CONTROLLER_MANAGER`, `SCHEDULER`, and `WORKLOADS`.
    #[builder(into)]
    #[serde(rename = "enableComponents")]
    pub r#enable_components: Box<Vec<String>>,
}
