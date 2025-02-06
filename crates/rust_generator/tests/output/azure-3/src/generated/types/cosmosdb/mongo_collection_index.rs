#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MongoCollectionIndex {
    /// Specifies the list of user settable keys for each Cosmos DB Mongo Collection.
    #[builder(into)]
    #[serde(rename = "keys")]
    pub r#keys: Box<Vec<String>>,
    /// Is the index unique or not? Defaults to `false`.
    /// 
    /// > **Note:** An index with an "_id" key must be specified.
    #[builder(into, default)]
    #[serde(rename = "unique")]
    pub r#unique: Box<Option<bool>>,
}
