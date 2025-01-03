#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDomainSettingsDockerSettings {
    /// Indicates whether the domain can access Docker. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "enableDockerAccess")]
    pub r#enable_docker_access: Box<Option<String>>,
    /// The list of Amazon Web Services accounts that are trusted when the domain is created in VPC-only mode.
    #[builder(into, default)]
    #[serde(rename = "vpcOnlyTrustedAccounts")]
    pub r#vpc_only_trusted_accounts: Box<Option<Vec<String>>>,
}
