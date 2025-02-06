#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTarget {
    /// BigQuery target for Discovery. The first target to match a table will be the one applied.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "bigQueryTarget")]
    pub r#big_query_target: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTarget>>,
    /// Cloud SQL target for Discovery. The first target to match a table will be the one applied.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudSqlTarget")]
    pub r#cloud_sql_target: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudSqlTarget>>,
    /// Cloud Storage target for Discovery. The first target to match a bucket will be the one applied.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudStorageTarget")]
    pub r#cloud_storage_target: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTarget>>,
    /// Discovery target that looks for credentials and secrets stored in cloud resource metadata and reports them as vulnerabilities to Security Command Center. Only one target of this type is allowed.
    #[builder(into, default)]
    #[serde(rename = "secretsTarget")]
    pub r#secrets_target: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetSecretsTarget>>,
}
