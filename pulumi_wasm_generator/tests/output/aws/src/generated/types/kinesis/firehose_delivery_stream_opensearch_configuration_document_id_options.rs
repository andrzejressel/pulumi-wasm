#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamOpensearchConfigurationDocumentIdOptions {
    /// The method for setting up document ID. Valid values: `FIREHOSE_DEFAULT`, `NO_DOCUMENT_ID`.
    #[builder(into)]
    #[serde(rename = "defaultDocumentIdFormat")]
    pub r#default_document_id_format: Box<String>,
}
