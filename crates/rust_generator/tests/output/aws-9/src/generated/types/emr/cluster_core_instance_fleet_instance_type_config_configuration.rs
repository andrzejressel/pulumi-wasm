#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterCoreInstanceFleetInstanceTypeConfigConfiguration {
    /// Classification within a configuration.
    #[builder(into, default)]
    #[serde(rename = "classification")]
    pub r#classification: Box<Option<String>>,
    /// Map of properties specified within a configuration classification.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}
