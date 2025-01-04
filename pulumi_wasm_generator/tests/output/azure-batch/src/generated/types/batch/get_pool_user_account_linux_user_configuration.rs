#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolUserAccountLinuxUserConfiguration {
    /// The user ID of the user account.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: Box<i32>,
    /// The SSH private key for the user account.
    #[builder(into)]
    #[serde(rename = "sshPrivateKey")]
    pub r#ssh_private_key: Box<String>,
    /// The group ID for the user account.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: Box<i32>,
}
