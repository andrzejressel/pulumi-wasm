#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamDestinationConfigGcsDestinationConfigJsonFileFormat {
    /// Compression of the loaded JSON file.
    /// Possible values are: `NO_COMPRESSION`, `GZIP`.
    #[builder(into, default)]
    #[serde(rename = "compression")]
    pub r#compression: Box<Option<String>>,
    /// The schema file format along JSON data files.
    /// Possible values are: `NO_SCHEMA_FILE`, `AVRO_SCHEMA_FILE`.
    #[builder(into, default)]
    #[serde(rename = "schemaFileFormat")]
    pub r#schema_file_format: Box<Option<String>>,
}
