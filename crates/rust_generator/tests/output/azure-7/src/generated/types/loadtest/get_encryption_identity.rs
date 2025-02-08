#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetEncryptionIdentity {
    /// The User Assigned Identity ID that is assigned to this Load Test Encryption.
    #[builder(into)]
    #[serde(rename = "identityId")]
    pub r#identity_id: Box<String>,
    /// Type of Managed Service Identity that is assigned to this Load Test Encryption.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
