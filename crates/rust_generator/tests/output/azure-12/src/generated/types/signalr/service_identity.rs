#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceIdentity {
    /// Specifies a list of User Assigned Managed Identity IDs to be assigned to this signalR.
    /// 
    /// > **NOTE:** This is required when `type` is set to `UserAssigned`
    #[builder(into, default)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
    /// Specifies the type of Managed Service Identity that should be configured on this signalR. Possible values are `SystemAssigned`, `UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
