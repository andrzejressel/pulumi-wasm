#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContainerCapabilities {
    /// List of linux capabilities to add.
    #[builder(into, default)]
    #[serde(rename = "adds")]
    pub r#adds: Box<Option<Vec<String>>>,
    /// List of linux capabilities to drop.
    #[builder(into, default)]
    #[serde(rename = "drops")]
    pub r#drops: Box<Option<Vec<String>>>,
}
