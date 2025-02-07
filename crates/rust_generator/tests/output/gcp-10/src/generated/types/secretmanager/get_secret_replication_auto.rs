#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecretReplicationAuto {
    /// The customer-managed encryption configuration of the Secret.
    /// If no configuration is provided, Google-managed default
    /// encryption is used.
    #[builder(into)]
    #[serde(rename = "customerManagedEncryptions")]
    pub r#customer_managed_encryptions: Box<Vec<super::super::types::secretmanager::GetSecretReplicationAutoCustomerManagedEncryption>>,
}
