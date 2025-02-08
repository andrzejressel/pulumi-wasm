#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAccountIdentity {
    #[builder(into, default)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Option<Vec<String>>>,
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
