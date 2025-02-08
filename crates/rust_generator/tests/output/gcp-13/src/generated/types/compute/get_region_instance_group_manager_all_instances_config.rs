#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRegionInstanceGroupManagerAllInstancesConfig {
    /// The label key-value pairs that you want to patch onto the instance,
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The metadata key-value pairs that you want to patch onto the instance. For more information, see Project and instance metadata,
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<std::collections::HashMap<String, String>>,
}
