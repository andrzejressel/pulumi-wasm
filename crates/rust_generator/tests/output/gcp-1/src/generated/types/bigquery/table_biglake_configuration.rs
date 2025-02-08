#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableBiglakeConfiguration {
    /// The connection specifying the credentials to be used to
    /// read and write to external storage, such as Cloud Storage. The connection_id can
    /// have the form "&lt;project\_id&gt;.&lt;location\_id&gt;.&lt;connection\_id&gt;" or
    /// projects/&lt;project\_id&gt;/locations/&lt;location\_id&gt;/connections/&lt;connection\_id&gt;".
    #[builder(into)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<String>,
    /// The file format the table data is stored in.
    #[builder(into)]
    #[serde(rename = "fileFormat")]
    pub r#file_format: Box<String>,
    /// The fully qualified location prefix of the external folder where table data
    /// is stored. The '*' wildcard character is not allowed. The URI should be in the format "gs://bucket/path_to_table/"
    #[builder(into)]
    #[serde(rename = "storageUri")]
    pub r#storage_uri: Box<String>,
    /// The table format the metadata only snapshots are stored in.
    #[builder(into)]
    #[serde(rename = "tableFormat")]
    pub r#table_format: Box<String>,
}
