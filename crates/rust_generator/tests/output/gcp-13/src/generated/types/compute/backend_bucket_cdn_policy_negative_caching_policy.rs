#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackendBucketCdnPolicyNegativeCachingPolicy {
    /// The HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501
    /// can be specified as values, and you cannot specify a status code more than once.
    #[builder(into, default)]
    #[serde(rename = "code")]
    pub r#code: Box<Option<i32>>,
    /// The TTL (in seconds) for which to cache responses with the corresponding status code. The maximum allowed value is 1800s
    /// (30 minutes), noting that infrequently accessed objects may be evicted from the cache before the defined TTL.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
}
