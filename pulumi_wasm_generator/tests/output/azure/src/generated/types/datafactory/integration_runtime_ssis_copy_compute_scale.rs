#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationRuntimeSsisCopyComputeScale {
    /// Specifies the data integration unit number setting reserved for copy activity execution. Supported values are multiples of `4` in range 4-256.
    #[builder(into, default)]
    #[serde(rename = "dataIntegrationUnit")]
    pub r#data_integration_unit: Box<Option<i32>>,
    /// Specifies the time to live (in minutes) setting of integration runtime which will execute copy activity. Possible values are at least `5`.
    #[builder(into, default)]
    #[serde(rename = "timeToLive")]
    pub r#time_to_live: Box<Option<i32>>,
}