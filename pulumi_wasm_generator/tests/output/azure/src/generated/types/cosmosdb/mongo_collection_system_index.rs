#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
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