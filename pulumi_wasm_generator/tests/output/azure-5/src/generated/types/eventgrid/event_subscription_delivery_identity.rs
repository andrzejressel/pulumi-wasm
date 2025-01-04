#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSubscriptionDeliveryIdentity {
    /// Specifies the type of Managed Service Identity that is used for event delivery. Allowed value is `SystemAssigned`, `UserAssigned`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The user identity associated with the resource.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentity")]
    pub r#user_assigned_identity: Box<Option<String>>,
}
