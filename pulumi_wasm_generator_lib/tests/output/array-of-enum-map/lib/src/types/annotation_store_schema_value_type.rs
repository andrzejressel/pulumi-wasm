#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
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
