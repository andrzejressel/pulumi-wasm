#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLaunchTemplateCpuOption {
    #[builder(into)]
    #[serde(rename = "amdSevSnp")]
    pub r#amd_sev_snp: Box<String>,
    #[builder(into)]
    #[serde(rename = "coreCount")]
    pub r#core_count: Box<i32>,
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: Box<i32>,
}
