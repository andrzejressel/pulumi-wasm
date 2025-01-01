#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HBaseClusterMetastoresAmbari {
    /// The external Hive metastore's existing SQL database. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// The external Ambari metastore's existing SQL server admin password. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The fully-qualified domain name (FQDN) of the SQL server to use for the external Ambari metastore. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: Box<String>,
    /// The external Ambari metastore's existing SQL server admin username. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
