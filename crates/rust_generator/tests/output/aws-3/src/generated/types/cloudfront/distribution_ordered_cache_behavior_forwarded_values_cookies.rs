#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionOrderedCacheBehaviorForwardedValuesCookies {
    /// Whether you want CloudFront to forward cookies to the origin that is associated with this cache behavior. You can specify `all`, `none` or `whitelist`. If `whitelist`, you must include the subsequent `whitelisted_names`.
    #[builder(into)]
    #[serde(rename = "forward")]
    pub r#forward: Box<String>,
    /// If you have specified `whitelist` to `forward`, the whitelisted cookies that you want CloudFront to forward to your origin.
    #[builder(into, default)]
    #[serde(rename = "whitelistedNames")]
    pub r#whitelisted_names: Box<Option<Vec<String>>>,
}
