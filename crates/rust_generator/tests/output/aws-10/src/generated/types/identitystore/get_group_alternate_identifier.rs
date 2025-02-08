#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetGroupAlternateIdentifier {
    /// Configuration block for filtering by the identifier issued by an external identity provider. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "externalId")]
    pub r#external_id: Box<Option<super::super::types::identitystore::GetGroupAlternateIdentifierExternalId>>,
    /// An entity attribute that's unique to a specific entity. Detailed below.
    /// 
    /// > Exactly one of the above arguments must be provided.
    #[builder(into, default)]
    #[serde(rename = "uniqueAttribute")]
    pub r#unique_attribute: Box<Option<super::super::types::identitystore::GetGroupAlternateIdentifierUniqueAttribute>>,
}
