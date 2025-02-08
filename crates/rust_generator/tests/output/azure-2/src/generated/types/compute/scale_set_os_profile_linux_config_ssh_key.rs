#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScaleSetOsProfileLinuxConfigSshKey {
    /// The Public SSH Key which should be written to the `path` defined above.
    /// 
    /// > **Note:** Azure only supports RSA SSH2 key signatures of at least 2048 bits in length
    /// 
    /// > **NOTE:** Rather than defining this in-line you can source this from a local file using the `file` function - for example `key_data = file("~/.ssh/id_rsa.pub")`.
    #[builder(into, default)]
    #[serde(rename = "keyData")]
    pub r#key_data: Box<Option<String>>,
    /// The path of the destination file on the virtual machine
    /// 
    /// > **NOTE:** Due to a limitation in the Azure VM Agent the only allowed `path` is `/home/{username}/.ssh/authorized_keys`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
