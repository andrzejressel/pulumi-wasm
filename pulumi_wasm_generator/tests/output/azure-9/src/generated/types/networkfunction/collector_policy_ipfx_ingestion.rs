#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CollectorPolicyIpfxIngestion {
    /// A list of ingestion source resource IDs. Changing this forces a new Network Function Collector Policy to be created.
    #[builder(into)]
    #[serde(rename = "sourceResourceIds")]
    pub r#source_resource_ids: Box<Vec<String>>,
}
