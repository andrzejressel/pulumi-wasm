#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnterpriseKeyIosSettings {
    /// If set to true, it means allowed_bundle_ids will not be enforced.
    #[builder(into, default)]
    #[serde(rename = "allowAllBundleIds")]
    pub r#allow_all_bundle_ids: Box<Option<bool>>,
    /// iOS bundle ids of apps allowed to use the key. Example: 'com.companyname.productname.appname'
    #[builder(into, default)]
    #[serde(rename = "allowedBundleIds")]
    pub r#allowed_bundle_ids: Box<Option<Vec<String>>>,
}
