#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceIdentity {
    /// The list of User Assigned Managed Identity IDs assigned to this API Management Service.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The Principal ID of the System Assigned Managed Service Identity that is configured on this API Management Service.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The ID of the Tenant which has access to this API Management instance.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The type of Managed Service Identity that is configured on this API Management Service.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
