#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceNode {
    /// Node identifying string. e.g. 'node-0', 'node-1'
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Location of the node.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Box<String>,
}
