#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRestorableDatabaseAccountsAccountRestorableLocation {
    /// The creation time of the regional Cosmos DB Restorable Database Account.
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Box<String>,
    /// The deletion time of the regional Cosmos DB Restorable Database Account.
    #[builder(into)]
    #[serde(rename = "deletionTime")]
    pub r#deletion_time: Box<String>,
    /// The location where the Cosmos DB Database Account.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The instance ID of the regional Cosmos DB Restorable Database Account.
    #[builder(into)]
    #[serde(rename = "regionalDatabaseAccountInstanceId")]
    pub r#regional_database_account_instance_id: Box<String>,
}
