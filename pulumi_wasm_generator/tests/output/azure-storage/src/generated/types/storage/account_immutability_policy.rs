#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountImmutabilityPolicy {
    /// When enabled, new blocks can be written to an append blob while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted.
    #[builder(into)]
    #[serde(rename = "allowProtectedAppendWrites")]
    pub r#allow_protected_append_writes: Box<bool>,
    /// The immutability period for the blobs in the container since the policy creation, in days.
    #[builder(into)]
    #[serde(rename = "periodSinceCreationInDays")]
    pub r#period_since_creation_in_days: Box<i32>,
    /// Defines the mode of the policy. `Disabled` state disables the policy, `Unlocked` state allows increase and decrease of immutability retention time and also allows toggling allowProtectedAppendWrites property, `Locked` state only allows the increase of the immutability retention time. A policy can only be created in a Disabled or Unlocked state and can be toggled between the two states. Only a policy in an Unlocked state can transition to a Locked state which cannot be reverted.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
