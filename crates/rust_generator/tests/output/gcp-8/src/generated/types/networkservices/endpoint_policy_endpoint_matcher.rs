#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointPolicyEndpointMatcher {
    /// The matcher is based on node metadata presented by xDS clients.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metadataLabelMatcher")]
    pub r#metadata_label_matcher: Box<super::super::types::networkservices::EndpointPolicyEndpointMatcherMetadataLabelMatcher>,
}
