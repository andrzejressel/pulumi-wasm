#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReference {
    /// Name of the customer managed IAM Policy to be attached.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The path to the IAM policy to be attached. The default is `/`. See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-friendly-names) for more information.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
