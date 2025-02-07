#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSpringCloudServiceConfigServerGitSettingSshAuth {
    /// The host key of the Git repository server.
    #[builder(into)]
    #[serde(rename = "hostKey")]
    pub r#host_key: Box<String>,
    /// The host key algorithm.
    #[builder(into)]
    #[serde(rename = "hostKeyAlgorithm")]
    pub r#host_key_algorithm: Box<String>,
    /// The SSH private key to access the Git repository, needed when the URI starts with `git@` or `ssh://`.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<String>,
    /// Indicates whether the Config Server instance will fail to start if the host_key does not match.
    #[builder(into)]
    #[serde(rename = "strictHostKeyCheckingEnabled")]
    pub r#strict_host_key_checking_enabled: Box<bool>,
}
