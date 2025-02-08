#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityPolicyRuleHeaderAction {
    /// The list of request headers to add or overwrite if they're already present.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestHeadersToAdds")]
    pub r#request_headers_to_adds: Box<Option<Vec<super::super::types::compute::SecurityPolicyRuleHeaderActionRequestHeadersToAdd>>>,
}
