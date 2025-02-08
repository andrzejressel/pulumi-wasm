#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MigrationJobReverseSshConnectivity {
    /// The name of the virtual machine (Compute Engine) used as the bastion server
    /// for the SSH tunnel.
    #[builder(into, default)]
    #[serde(rename = "vm")]
    pub r#vm: Box<Option<String>>,
    /// The IP of the virtual machine (Compute Engine) used as the bastion server
    /// for the SSH tunnel.
    #[builder(into, default)]
    #[serde(rename = "vmIp")]
    pub r#vm_ip: Box<Option<String>>,
    /// The forwarding port of the virtual machine (Compute Engine) used as the
    /// bastion server for the SSH tunnel.
    #[builder(into, default)]
    #[serde(rename = "vmPort")]
    pub r#vm_port: Box<Option<i32>>,
    /// The name of the VPC to peer with the Cloud SQL private network.
    #[builder(into, default)]
    #[serde(rename = "vpc")]
    pub r#vpc: Box<Option<String>>,
}
