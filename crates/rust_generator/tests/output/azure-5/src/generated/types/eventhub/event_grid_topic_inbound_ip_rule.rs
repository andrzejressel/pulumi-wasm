#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventGridTopicInboundIpRule {
    /// The action to take when the rule is matched. Possible values are `Allow`. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// The IP mask (CIDR) to match on.
    #[builder(into)]
    #[serde(rename = "ipMask")]
    pub r#ip_mask: Box<String>,
}
