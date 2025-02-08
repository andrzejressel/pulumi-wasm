#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupAdditionalGroupKey {
    /// (Output)
    /// The ID of the entity.
    /// For Google-managed entities, the id must be the email address of an existing
    /// group or user.
    /// For external-identity-mapped entities, the id must be a string conforming
    /// to the Identity Source's requirements.
    /// Must be unique within a namespace.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// (Output)
    /// The namespace in which the entity exists.
    /// If not specified, the EntityKey represents a Google-managed entity
    /// such as a Google user or a Google Group.
    /// If specified, the EntityKey represents an external-identity-mapped group.
    /// The namespace must correspond to an identity source created in Admin Console
    /// and must be in the form of `identitysources/{identity_source_id}`.
    #[builder(into, default)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<Option<String>>,
}
