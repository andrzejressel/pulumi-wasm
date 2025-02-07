#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDiskEncryptionSetIdentity {
    /// A list of User Assigned Managed Identity IDs assigned to this Disk Encryption Set.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// The (Client) ID of the Service Principal.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The ID of the Tenant the Service Principal is assigned in.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The type of Managed Service Identity that is configured on this Disk Encryption Set.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
