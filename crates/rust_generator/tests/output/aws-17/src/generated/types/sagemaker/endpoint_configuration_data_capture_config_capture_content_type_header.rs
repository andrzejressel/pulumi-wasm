#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointConfigurationDataCaptureConfigCaptureContentTypeHeader {
    /// The CSV content type headers to capture.
    #[builder(into, default)]
    #[serde(rename = "csvContentTypes")]
    pub r#csv_content_types: Box<Option<Vec<String>>>,
    /// The JSON content type headers to capture.
    #[builder(into, default)]
    #[serde(rename = "jsonContentTypes")]
    pub r#json_content_types: Box<Option<Vec<String>>>,
}
