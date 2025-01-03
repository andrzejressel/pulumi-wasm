#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNodeGroupRemoteAccess {
    /// EC2 Key Pair name that provides access for SSH communication with the worker nodes in the EKS Node Group.
    #[builder(into)]
    #[serde(rename = "ec2SshKey")]
    pub r#ec_2_ssh_key: Box<String>,
    /// Set of EC2 Security Group IDs to allow SSH access (port 22) from on the worker nodes.
    #[builder(into)]
    #[serde(rename = "sourceSecurityGroupIds")]
    pub r#source_security_group_ids: Box<Vec<String>>,
}
