#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HttpRouteRuleActionRetryPolicy {
    /// Specifies the allowed number of retries.
    #[builder(into, default)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Box<Option<i32>>,
    /// Specifies a non-zero timeout per retry attempt. A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "perTryTimeout")]
    pub r#per_try_timeout: Box<Option<String>>,
    /// Specifies one or more conditions when this retry policy applies.
    #[builder(into, default)]
    #[serde(rename = "retryConditions")]
    pub r#retry_conditions: Box<Option<Vec<String>>>,
}
