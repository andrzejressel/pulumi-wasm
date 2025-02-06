#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountRestoreDatabase {
    /// A list of the collection names for the restore request. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "collectionNames")]
    pub r#collection_names: Box<Option<Vec<String>>>,
    /// The database name for the restore request. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
