#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StackInstancesDeploymentTargets {
    /// Limit deployment targets to individual accounts or include additional accounts with provided OUs. Valid values: `INTERSECTION`, `DIFFERENCE`, `UNION`, `NONE`.
    #[builder(into, default)]
    #[serde(rename = "accountFilterType")]
    pub r#account_filter_type: Box<Option<String>>,
    /// List of accounts to deploy stack set updates.
    #[builder(into, default)]
    #[serde(rename = "accounts")]
    pub r#accounts: Box<Option<Vec<String>>>,
    /// S3 URL of the file containing the list of accounts.
    #[builder(into, default)]
    #[serde(rename = "accountsUrl")]
    pub r#accounts_url: Box<Option<String>>,
    /// Organization root ID or organizational unit (OU) IDs to which stack sets deploy.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnitIds")]
    pub r#organizational_unit_ids: Box<Option<Vec<String>>>,
}
