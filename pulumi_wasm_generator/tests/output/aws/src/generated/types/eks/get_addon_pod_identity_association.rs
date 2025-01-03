#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAddonPodIdentityAssociation {
    /// ARN of the IAM role associated with the EKS add-on.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// Service account associated with the EKS add-on.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<String>,
}
