#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WebAclDefaultAction {
    /// Specifies that AWS WAF should allow requests by default. See `allow` below for details.
    #[builder(into, default)]
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<super::super::types::wafv2::WebAclDefaultActionAllow>>,
    /// Specifies that AWS WAF should block requests by default. See `block` below for details.
    #[builder(into, default)]
    #[serde(rename = "block")]
    pub r#block: Box<Option<super::super::types::wafv2::WebAclDefaultActionBlock>>,
}
