#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountCapacity {
    /// The total throughput limit imposed on this Cosmos DB account (RU/s). Possible values are at least `-1`. `-1` means no limit.
    #[builder(into)]
    #[serde(rename = "totalThroughputLimit")]
    pub r#total_throughput_limit: Box<i32>,
}
