#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GrpcRouteRuleActionRetryPolicy {
    /// Specifies the allowed number of retries.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Box<Option<i32>>,
    /// Specifies one or more conditions when this retry policy applies.
    /// Each value may be one of: `connect-failure`, `refused-stream`, `cancelled`, `deadline-exceeded`, `resource-exhausted`, `unavailable`.
    #[builder(into, default)]
    #[serde(rename = "retryConditions")]
    pub r#retry_conditions: Box<Option<Vec<String>>>,
}
