#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogDatabaseCreateTableDefaultPermissionPrincipal {
    /// An identifier for the Lake Formation principal.
    #[builder(into, default)]
    #[serde(rename = "dataLakePrincipalIdentifier")]
    pub r#data_lake_principal_identifier: Box<Option<String>>,
}
