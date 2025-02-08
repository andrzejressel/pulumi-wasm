#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCoreNetworkPolicyDocumentSegmentActionWhenSentTo {
    /// A list of strings. The list of segments that the `send-via` `action` uses.
    #[builder(into, default)]
    #[serde(rename = "segments")]
    pub r#segments: Box<Option<Vec<String>>>,
}
