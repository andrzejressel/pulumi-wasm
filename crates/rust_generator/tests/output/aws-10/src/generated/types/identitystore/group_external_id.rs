#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupExternalId {
    /// The identifier issued to this resource by an external identity provider.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The issuer for an external identifier.
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
}
