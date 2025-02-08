#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceDeploymentScriptAzureCliIdentity {
    /// Specifies the list of user-assigned managed identity IDs associated with the resource. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "identityIds")]
    pub r#identity_ids: Box<Vec<String>>,
    /// Type of the managed identity. The only possible value is `UserAssigned`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
