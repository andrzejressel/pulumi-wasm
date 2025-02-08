#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DicomStoreStreamConfig {
    /// BigQueryDestination to include a fully qualified BigQuery table URI where DICOM instance metadata will be streamed.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigqueryDestination")]
    pub r#bigquery_destination: Box<super::super::types::healthcare::DicomStoreStreamConfigBigqueryDestination>,
}
