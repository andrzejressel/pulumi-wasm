#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupLookupGroupKey {
    /// (Required) The ID of the entity.
    /// For Google-managed entities, the id is the email address of an existing group or user.
    /// For external-identity-mapped entities, the id is a string conforming
    /// to the Identity Source's requirements.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// (Optional) The namespace in which the entity exists.
    /// If not populated, the EntityKey represents a Google-managed entity
    /// such as a Google user or a Google Group.
    /// If populated, the EntityKey represents an external-identity-mapped group.
    /// The namespace must correspond to an identity source created in Admin Console
    /// and must be in the form of `identitysources/{identity_source_id}`.
    #[builder(into, default)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<Option<String>>,
}
