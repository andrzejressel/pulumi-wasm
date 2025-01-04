#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DicomStoreStreamConfigBigqueryDestination {
    /// a fully qualified BigQuery table URI where DICOM instance metadata will be streamed.
    #[builder(into)]
    #[serde(rename = "tableUri")]
    pub r#table_uri: Box<String>,
}
