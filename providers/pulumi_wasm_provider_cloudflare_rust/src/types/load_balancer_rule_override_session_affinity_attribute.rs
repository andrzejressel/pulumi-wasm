#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerRuleOverrideSessionAffinityAttribute {
    /// Configures the HTTP header names to use when header session affinity is enabled.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<String>>>,
    /// Configures how headers are used when header session affinity is enabled. Set to true to require all headers to be present on requests in order for sessions to be created or false to require at least one header to be present. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Box<Option<bool>>,
    /// Configures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set `secure="Never"`. Available values: `Auto`, `Lax`, `None`, `Strict`.
    #[builder(into, default)]
    #[serde(rename = "samesite")]
    pub r#samesite: Box<Option<String>>,
    /// Configures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`.
    #[builder(into, default)]
    #[serde(rename = "secure")]
    pub r#secure: Box<Option<String>>,
    /// Configures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`.
    #[builder(into, default)]
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Box<Option<String>>,
}
