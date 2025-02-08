#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseHiveOptions {
    /// Cloud Storage folder URI where the database data is stored, starting with "gs://".
    #[builder(into, default)]
    #[serde(rename = "locationUri")]
    pub r#location_uri: Box<Option<String>>,
    /// Stores user supplied Hive database parameters. An object containing a
    /// list of"key": value pairs.
    /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
}
