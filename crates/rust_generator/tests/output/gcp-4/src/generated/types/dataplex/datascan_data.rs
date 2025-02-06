#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatascanData {
    /// The Dataplex entity that represents the data source(e.g. BigQuery table) for Datascan.
    #[builder(into, default)]
    #[serde(rename = "entity")]
    pub r#entity: Box<Option<String>>,
    /// The service-qualified full resource name of the cloud resource for a DataScan job to scan against. The field could be:
    /// (Cloud Storage bucket for DataDiscoveryScan)BigQuery table of type "TABLE" for DataProfileScan/DataQualityScan.
    #[builder(into, default)]
    #[serde(rename = "resource")]
    pub r#resource: Box<Option<String>>,
}
