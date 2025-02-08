#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationSetTrackingOptions {
    /// Custom subdomain that is used to redirect email recipients to the Amazon SES event tracking domain.
    #[builder(into, default)]
    #[serde(rename = "customRedirectDomain")]
    pub r#custom_redirect_domain: Box<Option<String>>,
}
