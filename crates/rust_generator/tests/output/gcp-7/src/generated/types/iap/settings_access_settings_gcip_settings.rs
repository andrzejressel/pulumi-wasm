#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SettingsAccessSettingsGcipSettings {
    /// Login page URI associated with the GCIP tenants. Typically, all resources within
    /// the same project share the same login page, though it could be overridden at the
    /// sub resource level.
    #[builder(into, default)]
    #[serde(rename = "loginPageUri")]
    pub r#login_page_uri: Box<Option<String>>,
    /// GCIP tenant ids that are linked to the IAP resource. tenantIds could be a string
    /// beginning with a number character to indicate authenticating with GCIP tenant flow,
    /// or in the format of _ to indicate authenticating with GCIP agent flow. If agent flow
    /// is used, tenantIds should only contain one single element, while for tenant flow,
    /// tenantIds can contain multiple elements.
    #[builder(into, default)]
    #[serde(rename = "tenantIds")]
    pub r#tenant_ids: Box<Option<Vec<String>>>,
}
