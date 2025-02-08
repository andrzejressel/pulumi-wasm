#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EndpointPolicyEndpointMatcherMetadataLabelMatcherMetadataLabel {
    /// Required. Label name presented as key in xDS Node Metadata.
    #[builder(into)]
    #[serde(rename = "labelName")]
    pub r#label_name: Box<String>,
    /// Required. Label value presented as value corresponding to the above key, in xDS Node Metadata.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "labelValue")]
    pub r#label_value: Box<String>,
}
