#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamDestinationConfigGcsDestinationConfig {
    /// AVRO file format configuration.
    #[builder(into, default)]
    #[serde(rename = "avroFileFormat")]
    pub r#avro_file_format: Box<Option<super::super::types::datastream::StreamDestinationConfigGcsDestinationConfigAvroFileFormat>>,
    /// The maximum duration for which new events are added before a file is closed and a new file is created.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s". Defaults to 900s.
    #[builder(into, default)]
    #[serde(rename = "fileRotationInterval")]
    pub r#file_rotation_interval: Box<Option<String>>,
    /// The maximum file size to be saved in the bucket.
    #[builder(into, default)]
    #[serde(rename = "fileRotationMb")]
    pub r#file_rotation_mb: Box<Option<i32>>,
    /// JSON file format configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "jsonFileFormat")]
    pub r#json_file_format: Box<Option<super::super::types::datastream::StreamDestinationConfigGcsDestinationConfigJsonFileFormat>>,
    /// Path inside the Cloud Storage bucket to write data to.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
