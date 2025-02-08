#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MetadataSupport {
    /// The email address of the support contact.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// The link for support help.
    #[builder(into, default)]
    #[serde(rename = "link")]
    pub r#link: Box<Option<String>>,
    /// The name of the support contact.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The type of support for content item. Possible values are `Microsoft`, `Partner` and `Community`.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
