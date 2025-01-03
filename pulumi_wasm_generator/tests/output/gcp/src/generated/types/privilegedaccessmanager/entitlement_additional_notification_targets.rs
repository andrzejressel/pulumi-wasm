#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EntitlementAdditionalNotificationTargets {
    /// Optional. Additional email addresses to be notified when a principal(requester) is granted access.
    #[builder(into, default)]
    #[serde(rename = "adminEmailRecipients")]
    pub r#admin_email_recipients: Box<Option<Vec<String>>>,
    /// Optional. Additional email address to be notified about an eligible entitlement.
    #[builder(into, default)]
    #[serde(rename = "requesterEmailRecipients")]
    pub r#requester_email_recipients: Box<Option<Vec<String>>>,
}
