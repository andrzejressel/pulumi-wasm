#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoadTestEncryptionIdentity {
    /// The User Assigned Identity ID that should be assigned to this Load Test Encryption. Changing this forces a new Load Test to be created.
    #[builder(into)]
    #[serde(rename = "identityId")]
    pub r#identity_id: Box<String>,
    /// Specifies the type of Managed Identity that should be assigned to this Load Test Encryption. Possible values are `SystemAssigned` or `UserAssigned`. Changing this forces a new Load Test to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
