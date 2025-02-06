#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecurityPolicyRuleRateLimitOptions {
    /// Can only be specified if the action for the rule is "rate_based_ban".
    /// If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold.
    #[builder(into, default)]
    #[serde(rename = "banDurationSec")]
    pub r#ban_duration_sec: Box<Option<i32>>,
    /// Can only be specified if the action for the rule is "rate_based_ban".
    /// If specified, the key will be banned for the configured 'banDurationSec' when the number of requests that exceed the 'rateLimitThreshold' also exceed this 'banThreshold'.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "banThreshold")]
    pub r#ban_threshold: Box<Option<super::super::types::compute::SecurityPolicyRuleRateLimitOptionsBanThreshold>>,
    /// Action to take for requests that are under the configured rate limit threshold.
    /// Valid option is "allow" only.
    #[builder(into, default)]
    #[serde(rename = "conformAction")]
    pub r#conform_action: Box<Option<String>>,
    /// Determines the key to enforce the rateLimitThreshold on. Possible values are:
    /// * ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if "enforceOnKey" is not configured.
    /// * IP: The source IP address of the request is the key. Each IP has this limit enforced separately.
    /// * HTTP_HEADER: The value of the HTTP header whose name is configured under "enforceOnKeyName". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.
    /// * XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.
    /// * HTTP_COOKIE: The value of the HTTP cookie whose name is configured under "enforceOnKeyName". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.
    /// * HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.
    /// * SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.
    /// * REGION_CODE: The country/region from which the request originates.
    /// * TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.
    /// * USER_IP: The IP address of the originating client, which is resolved based on "userIpRequestHeaders" configured with the security policy. If there is no "userIpRequestHeaders" configuration or an IP address cannot be resolved from it, the key type defaults to IP.
    /// Possible values are: `ALL`, `IP`, `HTTP_HEADER`, `XFF_IP`, `HTTP_COOKIE`, `HTTP_PATH`, `SNI`, `REGION_CODE`, `TLS_JA3_FINGERPRINT`, `USER_IP`.
    #[builder(into, default)]
    #[serde(rename = "enforceOnKey")]
    pub r#enforce_on_key: Box<Option<String>>,
    /// If specified, any combination of values of enforceOnKeyType/enforceOnKeyName is treated as the key on which ratelimit threshold/action is enforced.
    /// You can specify up to 3 enforceOnKeyConfigs.
    /// If enforceOnKeyConfigs is specified, enforceOnKey must not be specified.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "enforceOnKeyConfigs")]
    pub r#enforce_on_key_configs: Box<Option<Vec<super::super::types::compute::SecurityPolicyRuleRateLimitOptionsEnforceOnKeyConfig>>>,
    /// Rate limit key name applicable only for the following key types:
    /// HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.
    /// HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value.
    #[builder(into, default)]
    #[serde(rename = "enforceOnKeyName")]
    pub r#enforce_on_key_name: Box<Option<String>>,
    /// Action to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint.
    /// Valid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502.
    #[builder(into, default)]
    #[serde(rename = "exceedAction")]
    pub r#exceed_action: Box<Option<String>>,
    /// Parameters defining the redirect action that is used as the exceed action. Cannot be specified if the exceed action is not redirect. This field is only supported in Global Security Policies of type CLOUD_ARMOR.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "exceedRedirectOptions")]
    pub r#exceed_redirect_options: Box<Option<super::super::types::compute::SecurityPolicyRuleRateLimitOptionsExceedRedirectOptions>>,
    /// Threshold at which to begin ratelimiting.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "rateLimitThreshold")]
    pub r#rate_limit_threshold: Box<Option<super::super::types::compute::SecurityPolicyRuleRateLimitOptionsRateLimitThreshold>>,
}
