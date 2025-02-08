#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MongoRoleDefinitionPrivilegeResource {
    /// The name of the Mongo DB Collection that the Role Definition is applied.
    #[builder(into, default)]
    #[serde(rename = "collectionName")]
    pub r#collection_name: Box<Option<String>>,
    /// The name of the Mongo DB that the Role Definition is applied.
    #[builder(into, default)]
    #[serde(rename = "dbName")]
    pub r#db_name: Box<Option<String>>,
}
