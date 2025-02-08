#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetManagedInstanceIdentity {
    /// A list of User Assigned Managed Identity IDs assigned with the Identity of this SQL Managed Instance.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The Principal ID for the Service Principal associated with the Identity of this SQL Managed Instance.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID for the Service Principal associated with the Identity of this SQL Managed Instance.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The identity type of the SQL Managed Instance.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
