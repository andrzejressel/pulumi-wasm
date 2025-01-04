#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsNodePoolConfigSshConfig {
    /// The name of the EC2 key pair used to login into cluster machines.
    #[builder(into)]
    #[serde(rename = "ec2KeyPair")]
    pub r#ec_2_key_pair: Box<String>,
}
