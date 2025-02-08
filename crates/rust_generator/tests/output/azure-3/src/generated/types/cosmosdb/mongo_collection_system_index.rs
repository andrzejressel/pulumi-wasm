#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MongoCollectionSystemIndex {
    /// The list of system keys which are not settable for each Cosmos DB Mongo Collection.
    #[builder(into, default)]
    #[serde(rename = "keys")]
    pub r#keys: Box<Option<Vec<String>>>,
    /// Identifies whether the table contains no duplicate values.
    #[builder(into, default)]
    #[serde(rename = "unique")]
    pub r#unique: Box<Option<bool>>,
}
