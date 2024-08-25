#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LoadBalancerPoolOrigin {
    /// The IP address (IPv4 or IPv6) of the origin, or the publicly addressable hostname.
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// Whether this origin is enabled. Disabled origins will not receive traffic and are excluded from health checks. Defaults to `true`.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// HTTP request headers.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::LoadBalancerPoolOriginHeader>>>,
    /// A human-identifiable name for the origin.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The weight (0.01 - 1.00) of this origin, relative to other origins in the pool. Equal values mean equal weighting. A weight of 0 means traffic will not be sent to this origin, but health is still checked. When `origin_steering.policy="least_outstanding_requests"`, weight is used to scale the origin's outstanding requests. When `origin_steering.policy="least_connections"`, weight is used to scale the origin's open connections. Defaults to `1`.
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<f64>>,
}
