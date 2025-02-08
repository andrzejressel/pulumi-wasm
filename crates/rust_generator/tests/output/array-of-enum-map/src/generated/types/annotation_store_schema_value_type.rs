#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub enum AnnotationStoreSchemaValueType {
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "INT")]
    Int,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "FLOAT")]
    Float,
    #[serde(rename = "DOUBLE")]
    Double,
    #[serde(rename = "BOOLEAN")]
    Boolean,
}
