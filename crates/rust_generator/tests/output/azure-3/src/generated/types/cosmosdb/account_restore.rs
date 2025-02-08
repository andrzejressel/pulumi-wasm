#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountRestore {
    /// A `database` block as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "databases")]
    pub r#databases: Box<Option<Vec<super::super::types::cosmosdb::AccountRestoreDatabase>>>,
    /// One or more `gremlin_database` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "gremlinDatabases")]
    pub r#gremlin_databases: Box<Option<Vec<super::super::types::cosmosdb::AccountRestoreGremlinDatabase>>>,
    /// The creation time of the database or the collection (Datetime Format `RFC 3339`). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "restoreTimestampInUtc")]
    pub r#restore_timestamp_in_utc: Box<String>,
    /// The resource ID of the restorable database account from which the restore has to be initiated. The example is `/subscriptions/{subscriptionId}/providers/Microsoft.DocumentDB/locations/{location}/restorableDatabaseAccounts/{restorableDatabaseAccountName}`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Any database account with `Continuous` type (live account or accounts deleted in last 30 days) is a restorable database account and there cannot be Create/Update/Delete operations on the restorable database accounts. They can only be read and retrieved by `azure.cosmosdb.getRestorableDatabaseAccounts`.
    #[builder(into)]
    #[serde(rename = "sourceCosmosdbAccountId")]
    pub r#source_cosmosdb_account_id: Box<String>,
    /// A list of specific tables available for restore. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "tablesToRestores")]
    pub r#tables_to_restores: Box<Option<Vec<String>>>,
}
