#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionCacheBehaviorSettingsForwardedCookies {
    /// The specific cookies to forward to your distribution's origin.
    #[builder(into, default)]
    #[serde(rename = "cookiesAllowLists")]
    pub r#cookies_allow_lists: Box<Option<Vec<String>>>,
    /// Specifies which cookies to forward to the distribution's origin for a cache behavior: all, none, or allow-list to forward only the cookies specified in the cookiesAllowList parameter.
    #[builder(into, default)]
    #[serde(rename = "option")]
    pub r#option: Box<Option<String>>,
}
