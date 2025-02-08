#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ComputeClusterIdentity {
    /// Specifies a list of User Assigned Managed Identity IDs to be assigned to this Machine Learning Compute Cluster. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** This is required when `type` is set to `UserAssigned` or `SystemAssigned, UserAssigned`.
    #[builder(into, default)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Option<Vec<String>>>,
    /// The Principal ID for the Service Principal associated with the Managed Service Identity of this Machine Learning Compute Cluster.
    #[builder(into, default)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<Option<String>>,
    /// The Tenant ID for the Service Principal associated with the Managed Service Identity of this Machine Learning Compute Cluster.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
    /// Specifies the type of Managed Service Identity that should be configured on this Machine Learning Compute Cluster. Possible values are `SystemAssigned`, `UserAssigned`, `SystemAssigned, UserAssigned` (to enable both). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
