#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConfigSmsRegionConfig {
    /// A policy of allowing SMS to every region by default and adding disallowed regions to a disallow list.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "allowByDefault")]
    pub r#allow_by_default: Box<Option<super::super::types::identityplatform::ConfigSmsRegionConfigAllowByDefault>>,
    /// A policy of only allowing regions by explicitly adding them to an allowlist.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "allowlistOnly")]
    pub r#allowlist_only: Box<Option<super::super::types::identityplatform::ConfigSmsRegionConfigAllowlistOnly>>,
}
