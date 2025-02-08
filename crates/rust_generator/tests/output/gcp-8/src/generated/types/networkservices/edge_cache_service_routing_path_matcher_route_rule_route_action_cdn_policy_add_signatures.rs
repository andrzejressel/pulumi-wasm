#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyAddSignatures {
    /// The actions to take to add signatures to responses.
    /// Each value may be one of: `GENERATE_COOKIE`, `GENERATE_TOKEN_HLS_COOKIELESS`, `PROPAGATE_TOKEN_HLS_COOKIELESS`.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<String>,
    /// The parameters to copy from the verified token to the generated token.
    /// Only the following parameters may be copied:
    /// * `PathGlobs`
    #[builder(into, default)]
    #[serde(rename = "copiedParameters")]
    pub r#copied_parameters: Box<Option<Vec<String>>>,
    /// The keyset to use for signature generation.
    /// The following are both valid paths to an EdgeCacheKeyset resource:
    /// * `projects/project/locations/global/edgeCacheKeysets/yourKeyset`
    /// * `yourKeyset`
    /// This must be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.  This field may not be specified otherwise.
    #[builder(into, default)]
    #[serde(rename = "keyset")]
    pub r#keyset: Box<Option<String>>,
    /// The query parameter in which to put the generated token.
    /// If not specified, defaults to `edge-cache-token`.
    /// If specified, the name must be 1-64 characters long and match the regular expression `a-zA-Z*` which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.
    /// This field may only be set when the GENERATE_TOKEN_HLS_COOKIELESS or PROPAGATE_TOKEN_HLS_COOKIELESS actions are specified.
    #[builder(into, default)]
    #[serde(rename = "tokenQueryParameter")]
    pub r#token_query_parameter: Box<Option<String>>,
    /// The duration the token is valid starting from the moment the token is first generated.
    /// Defaults to `86400s` (1 day).
    /// The TTL must be >= 0 and <= 604,800 seconds (1 week).
    /// This field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "tokenTtl")]
    pub r#token_ttl: Box<Option<String>>,
}
