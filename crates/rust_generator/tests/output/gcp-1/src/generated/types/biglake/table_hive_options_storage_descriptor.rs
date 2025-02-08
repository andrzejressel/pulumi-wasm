#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableHiveOptionsStorageDescriptor {
    /// The fully qualified Java class name of the input format.
    #[builder(into, default)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Box<Option<String>>,
    /// Cloud Storage folder URI where the table data is stored, starting with "gs://".
    #[builder(into, default)]
    #[serde(rename = "locationUri")]
    pub r#location_uri: Box<Option<String>>,
    /// The fully qualified Java class name of the output format.
    #[builder(into, default)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Box<Option<String>>,
}
