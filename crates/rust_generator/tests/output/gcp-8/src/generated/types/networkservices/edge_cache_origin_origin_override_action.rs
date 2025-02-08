#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EdgeCacheOriginOriginOverrideAction {
    /// The header actions, including adding and removing
    /// headers, for request handled by this origin.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Box<Option<super::super::types::networkservices::EdgeCacheOriginOriginOverrideActionHeaderAction>>,
    /// The URL rewrite configuration for request that are
    /// handled by this origin.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "urlRewrite")]
    pub r#url_rewrite: Box<Option<super::super::types::networkservices::EdgeCacheOriginOriginOverrideActionUrlRewrite>>,
}
