#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterLinuxProfile {
    /// The username associated with the administrator account of the Windows VMs.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: Box<String>,
    /// An `ssh_key` block as defined below.
    #[builder(into)]
    #[serde(rename = "sshKeys")]
    pub r#ssh_keys: Box<Vec<super::super::types::containerservice::GetKubernetesClusterLinuxProfileSshKey>>,
}