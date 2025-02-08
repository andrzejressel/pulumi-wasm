#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudServiceConfigServerGitSettingRepositorySshAuth {
    /// The host key of the Git repository server, should not include the algorithm prefix as covered by `host-key-algorithm`.
    #[builder(into, default)]
    #[serde(rename = "hostKey")]
    pub r#host_key: Box<Option<String>>,
    /// The host key algorithm, should be `ssh-dss`, `ssh-rsa`, `ecdsa-sha2-nistp256`, `ecdsa-sha2-nistp384`, or `ecdsa-sha2-nistp521`. Required only if `host-key` exists.
    #[builder(into, default)]
    #[serde(rename = "hostKeyAlgorithm")]
    pub r#host_key_algorithm: Box<Option<String>>,
    /// The SSH private key to access the Git repository, required when the URI starts with `git@` or `ssh://`.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<String>,
    /// Indicates whether the Config Server instance will fail to start if the host_key does not match. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "strictHostKeyCheckingEnabled")]
    pub r#strict_host_key_checking_enabled: Box<Option<bool>>,
}
