#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamBackfillAllMysqlExcludedObjects {
    /// MySQL databases on the server
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mysqlDatabases")]
    pub r#mysql_databases: Box<Vec<super::super::types::datastream::StreamBackfillAllMysqlExcludedObjectsMysqlDatabase>>,
}
