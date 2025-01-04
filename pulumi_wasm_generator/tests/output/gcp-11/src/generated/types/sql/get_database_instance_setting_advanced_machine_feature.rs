#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabaseInstanceSettingAdvancedMachineFeature {
    /// The number of threads per physical core. Can be 1 or 2.
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: Box<i32>,
}
