#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CollectorPolicyIpfxEmission {
    /// A list of emission destination types. The only possible value is `AzureMonitor`. Changing this forces a new Network Function Collector Policy to be created.
    #[builder(into)]
    #[serde(rename = "destinationTypes")]
    pub r#destination_types: Box<String>,
}