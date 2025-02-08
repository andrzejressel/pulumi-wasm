#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetConfigurationSetTrackingOption {
    /// The domain to use for tracking open and click events.
    #[builder(into)]
    #[serde(rename = "customRedirectDomain")]
    pub r#custom_redirect_domain: Box<String>,
    /// The https policy to use for tracking open and click events. Valid values are `REQUIRE`, `REQUIRE_OPEN_ONLY` or `OPTIONAL`.
    #[builder(into)]
    #[serde(rename = "httpsPolicy")]
    pub r#https_policy: Box<String>,
}
