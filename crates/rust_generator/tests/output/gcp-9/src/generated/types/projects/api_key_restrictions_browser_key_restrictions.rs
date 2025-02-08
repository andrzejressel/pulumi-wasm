#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiKeyRestrictionsBrowserKeyRestrictions {
    /// A list of regular expressions for the referrer URLs that are allowed to make API calls with this key.
    #[builder(into)]
    #[serde(rename = "allowedReferrers")]
    pub r#allowed_referrers: Box<Vec<String>>,
}
