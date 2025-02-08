#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BackupPolicyKubernetesClusterDefaultRetentionRuleLifeCycle {
    /// The type of data store. The only possible value is `OperationalStore`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "dataStoreType")]
    pub r#data_store_type: Box<String>,
    /// The retention duration up to which the backups are to be retained in the data stores. It should follow `ISO 8601` duration format. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
}
