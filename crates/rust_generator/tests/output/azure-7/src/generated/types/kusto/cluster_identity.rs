#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterIdentity {
    /// Specifies a list of User Assigned Managed Identity IDs to be assigned to this Kusto Cluster.
    /// 
    /// > **NOTE:** This is required when `type` is set to `UserAssigned` or `SystemAssigned, UserAssigned`.
    #[builder(into, default)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Option<Vec<String>>>,
    /// The Principal ID associated with this System Assigned Managed Service Identity.
    #[builder(into, default)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<Option<String>>,
    /// The Tenant ID associated with this System Assigned Managed Service Identity.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
    /// Specifies the type of Managed Service Identity that is configured on this Kusto Cluster. Possible values are: `SystemAssigned`, `UserAssigned` and `SystemAssigned, UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
