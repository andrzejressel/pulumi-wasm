#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionBuildConfigOnDeployUpdatePolicy {
    /// (Output)
    /// The runtime version which was used during latest function deployment.
    #[builder(into, default)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Box<Option<String>>,
}
