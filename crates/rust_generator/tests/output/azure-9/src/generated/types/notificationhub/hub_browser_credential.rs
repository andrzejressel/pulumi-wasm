#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HubBrowserCredential {
    /// The subject name of web push.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<String>,
    /// The Voluntary Application Server Identification (VAPID) private key.
    #[builder(into)]
    #[serde(rename = "vapidPrivateKey")]
    pub r#vapid_private_key: Box<String>,
    /// The Voluntary Application Server Identification (VAPID) public key.
    #[builder(into)]
    #[serde(rename = "vapidPublicKey")]
    pub r#vapid_public_key: Box<String>,
}
