#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum EngineMode {
    #[serde(rename = "provisioned")]
    Provisioned,
    #[serde(rename = "serverless")]
    Serverless,
    #[serde(rename = "parallelquery")]
    ParallelQuery,
    #[serde(rename = "global")]
    Global,
}
