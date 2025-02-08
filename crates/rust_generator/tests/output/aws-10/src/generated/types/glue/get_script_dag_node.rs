#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetScriptDagNode {
    /// Nested configuration an argument or property of a node. Defined below.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Box<Vec<super::super::types::glue::GetScriptDagNodeArg>>,
    /// Node identifier that is unique within the node's graph.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Line number of the node.
    #[builder(into, default)]
    #[serde(rename = "lineNumber")]
    pub r#line_number: Box<Option<i32>>,
    /// Type of node this is.
    #[builder(into)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Box<String>,
}
