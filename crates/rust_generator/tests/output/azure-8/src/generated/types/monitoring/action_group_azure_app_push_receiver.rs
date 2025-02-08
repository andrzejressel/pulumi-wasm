#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ActionGroupAzureAppPushReceiver {
    /// The email address of the user signed into the mobile app who will receive push notifications from this receiver.
    #[builder(into)]
    #[serde(rename = "emailAddress")]
    pub r#email_address: Box<String>,
    /// The name of the Azure app push receiver.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
