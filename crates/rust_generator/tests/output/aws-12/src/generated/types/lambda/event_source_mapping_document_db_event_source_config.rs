#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EventSourceMappingDocumentDbEventSourceConfig {
    /// The name of the collection to consume within the database. If you do not specify a collection, Lambda consumes all collections.
    #[builder(into, default)]
    #[serde(rename = "collectionName")]
    pub r#collection_name: Box<Option<String>>,
    /// The name of the database to consume within the DocumentDB cluster.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// Determines what DocumentDB sends to your event stream during document update operations. If set to `UpdateLookup`, DocumentDB sends a delta describing the changes, along with a copy of the entire document. Otherwise, DocumentDB sends only a partial document that contains the changes. Valid values: `UpdateLookup`, `Default`.
    #[builder(into, default)]
    #[serde(rename = "fullDocument")]
    pub r#full_document: Box<Option<String>>,
}
