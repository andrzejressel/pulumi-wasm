#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterBinaryAuthorization {
    /// Enable Binary Authorization for this cluster.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Mode of operation for Binary Authorization policy evaluation.
    #[builder(into)]
    #[serde(rename = "evaluationMode")]
    pub r#evaluation_mode: Box<String>,
}
