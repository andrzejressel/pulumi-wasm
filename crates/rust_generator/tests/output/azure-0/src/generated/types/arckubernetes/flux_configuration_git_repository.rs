#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FluxConfigurationGitRepository {
    /// Specifies the Base64-encoded HTTPS certificate authority contents used to access git private git repositories over HTTPS.
    #[builder(into, default)]
    #[serde(rename = "httpsCaCertBase64")]
    pub r#https_ca_cert_base_64: Box<Option<String>>,
    /// Specifies the Base64-encoded HTTPS personal access token or password that will be used to access the repository.
    #[builder(into, default)]
    #[serde(rename = "httpsKeyBase64")]
    pub r#https_key_base_64: Box<Option<String>>,
    /// Specifies the plaintext HTTPS username used to access private git repositories over HTTPS.
    #[builder(into, default)]
    #[serde(rename = "httpsUser")]
    pub r#https_user: Box<Option<String>>,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets. It must be between 1 and 63 characters. It can contain only lowercase letters, numbers, and hyphens (-). It must start and end with a lowercase letter or number.
    #[builder(into, default)]
    #[serde(rename = "localAuthReference")]
    pub r#local_auth_reference: Box<Option<String>>,
    /// Specifies the source reference type for the GitRepository object. Possible values are `branch`, `commit`, `semver` and `tag`.
    #[builder(into)]
    #[serde(rename = "referenceType")]
    pub r#reference_type: Box<String>,
    /// Specifies the source reference value for the GitRepository object.
    #[builder(into)]
    #[serde(rename = "referenceValue")]
    pub r#reference_value: Box<String>,
    /// Specifies the Base64-encoded known_hosts value containing public SSH keys required to access private git repositories over SSH.
    #[builder(into, default)]
    #[serde(rename = "sshKnownHostsBase64")]
    pub r#ssh_known_hosts_base_64: Box<Option<String>>,
    /// Specifies the Base64-encoded SSH private key in PEM format.
    #[builder(into, default)]
    #[serde(rename = "sshPrivateKeyBase64")]
    pub r#ssh_private_key_base_64: Box<Option<String>>,
    /// Specifies the interval at which to re-reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into, default)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Box<Option<i32>>,
    /// Specifies the maximum time to attempt to reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into, default)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<Option<i32>>,
    /// Specifies the URL to sync for the flux configuration git repository. It must start with `http://`, `https://`, `git@` or `ssh://`.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
