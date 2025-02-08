#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDomainInboundIpRule {
    /// The action to take when the rule is matched. Possible values are `Allow`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The IP mask (CIDR) to match on.
    #[builder(into)]
    #[serde(rename = "ipMask")]
    pub r#ip_mask: Box<String>,
}
