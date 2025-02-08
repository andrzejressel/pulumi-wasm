#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterKeyVaultSecretsProvider {
    /// An `secret_identity` block is exported. The exported attributes are defined below.
    #[builder(into, default)]
    #[serde(rename = "secretIdentities")]
    pub r#secret_identities: Box<Option<Vec<super::super::types::containerservice::KubernetesClusterKeyVaultSecretsProviderSecretIdentity>>>,
    /// Should the secret store CSI driver on the AKS cluster be enabled?
    #[builder(into, default)]
    #[serde(rename = "secretRotationEnabled")]
    pub r#secret_rotation_enabled: Box<Option<bool>>,
    /// The interval to poll for secret rotation. This attribute is only set when `secret_rotation` is true. Defaults to `2m`.
    /// 
    /// > **Note:** To enable`key_vault_secrets_provider` either `secret_rotation_enabled` or `secret_rotation_interval` must be specified.
    #[builder(into, default)]
    #[serde(rename = "secretRotationInterval")]
    pub r#secret_rotation_interval: Box<Option<String>>,
}
