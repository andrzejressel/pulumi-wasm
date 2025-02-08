#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HubEventHandlerAuth {
    /// Specify the identity ID of the target resource.
    /// 
    /// > **NOTE:** `managed_identity_id` is required if the auth block is defined
    #[builder(into)]
    #[serde(rename = "managedIdentityId")]
    pub r#managed_identity_id: Box<String>,
}
