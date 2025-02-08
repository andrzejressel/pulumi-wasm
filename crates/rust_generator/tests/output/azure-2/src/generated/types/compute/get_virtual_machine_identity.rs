#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualMachineIdentity {
    /// The list of User Managed Identity IDs which are assigned to the Virtual Machine.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The ID of the System Managed Service Principal assigned to the Virtual Machine.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The ID of the Tenant of the System Managed Service Principal assigned to the Virtual Machine.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The identity type of the Managed Identity assigned to the Virtual Machine.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
