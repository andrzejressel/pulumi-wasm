#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2VmSymptom {
    /// (Output)
    /// Timestamp when the Symptom is created.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// (Output)
    /// Detailed information of the current Symptom.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<String>>,
    /// (Output)
    /// Type of the Symptom.
    #[builder(into, default)]
    #[serde(rename = "symptomType")]
    pub r#symptom_type: Box<Option<String>>,
    /// (Output)
    /// A string used to uniquely distinguish a worker within a TPU node.
    #[builder(into, default)]
    #[serde(rename = "workerId")]
    pub r#worker_id: Box<Option<String>>,
}
