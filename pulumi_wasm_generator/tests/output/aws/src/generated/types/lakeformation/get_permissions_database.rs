#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPermissionsDatabase {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<String>,
    /// Name of the database resource. Unique to the Data Catalog.
    /// 
    /// The following argument is optional:
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}