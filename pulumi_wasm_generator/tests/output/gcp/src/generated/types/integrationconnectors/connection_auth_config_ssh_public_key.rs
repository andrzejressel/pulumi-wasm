#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionAuthConfigSshPublicKey {
    /// Format of SSH Client cert.
    #[builder(into, default)]
    #[serde(rename = "certType")]
    pub r#cert_type: Box<Option<String>>,
    /// SSH Client Cert. It should contain both public and private key.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sshClientCert")]
    pub r#ssh_client_cert: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigSshPublicKeySshClientCert>>,
    /// Password (passphrase) for ssh client certificate if it has one.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sshClientCertPass")]
    pub r#ssh_client_cert_pass: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigSshPublicKeySshClientCertPass>>,
    /// The user account used to authenticate.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
