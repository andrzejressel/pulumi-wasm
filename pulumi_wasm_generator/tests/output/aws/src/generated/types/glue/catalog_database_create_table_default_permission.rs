#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogDatabaseCreateTableDefaultPermission {
    /// The permissions that are granted to the principal.
    #[builder(into, default)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<Vec<String>>>,
    /// The principal who is granted permissions.. See `principal` below.
    #[builder(into, default)]
    #[serde(rename = "principal")]
    pub r#principal: Box<Option<super::super::types::glue::CatalogDatabaseCreateTableDefaultPermissionPrincipal>>,
}