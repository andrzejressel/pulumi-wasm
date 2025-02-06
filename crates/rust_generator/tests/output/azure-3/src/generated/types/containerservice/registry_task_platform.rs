#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryTaskPlatform {
    /// The OS architecture. Possible values are `amd64`, `x86`, `386`, `arm` and `arm64`.
    #[builder(into, default)]
    #[serde(rename = "architecture")]
    pub r#architecture: Box<Option<String>>,
    /// The operating system type required for the task. Possible values are `Windows` and `Linux`.
    #[builder(into)]
    #[serde(rename = "os")]
    pub r#os: Box<String>,
    /// The variant of the CPU. Possible values are `v6`, `v7`, `v8`.
    #[builder(into, default)]
    #[serde(rename = "variant")]
    pub r#variant: Box<Option<String>>,
}
