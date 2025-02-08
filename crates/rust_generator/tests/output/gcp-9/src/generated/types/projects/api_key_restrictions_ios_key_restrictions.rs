#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiKeyRestrictionsIosKeyRestrictions {
    /// A list of bundle IDs that are allowed when making API calls with this key.
    #[builder(into)]
    #[serde(rename = "allowedBundleIds")]
    pub r#allowed_bundle_ids: Box<Vec<String>>,
}
