#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuestPoliciesPackageRepositoryApt {
    /// Type of archive files in this repository. The default behavior is DEB.
    /// Default value is `DEB`.
    /// Possible values are: `DEB`, `DEB_SRC`.
    #[builder(into, default)]
    #[serde(rename = "archiveType")]
    pub r#archive_type: Box<Option<String>>,
    /// List of components for this repository. Must contain at least one item.
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Box<Vec<String>>,
    /// Distribution of this repository.
    #[builder(into)]
    #[serde(rename = "distribution")]
    pub r#distribution: Box<String>,
    /// URI of the key file for this repository. The agent maintains a keyring at
    /// /etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg containing all the keys in any applied guest policy.
    #[builder(into, default)]
    #[serde(rename = "gpgKey")]
    pub r#gpg_key: Box<Option<String>>,
    /// URI for this repository.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
