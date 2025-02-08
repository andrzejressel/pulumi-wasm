#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EntryBigqueryTableSpecViewSpec {
    /// (Output)
    /// The query that defines the table view.
    #[builder(into, default)]
    #[serde(rename = "viewQuery")]
    pub r#view_query: Box<Option<String>>,
}
