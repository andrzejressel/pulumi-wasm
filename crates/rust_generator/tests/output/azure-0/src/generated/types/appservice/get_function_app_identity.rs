#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFunctionAppIdentity {
    /// A list of User Assigned Identity IDs assigned to the Function App.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The ID of the Managed Identity assigned to the Function App.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The ID of the Tenant where the Managed Identity assigned to the Function App is located.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The identity type of the Managed Identity assigned to the Function App.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
