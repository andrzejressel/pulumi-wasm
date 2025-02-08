#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CatalogDatabaseCreateTableDefaultPermissionPrincipal {
    /// An identifier for the Lake Formation principal.
    #[builder(into, default)]
    #[serde(rename = "dataLakePrincipalIdentifier")]
    pub r#data_lake_principal_identifier: Box<Option<String>>,
}
