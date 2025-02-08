#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StackAccessEndpoint {
    /// Type of the interface endpoint.
    /// See the [`AccessEndpoint` AWS API documentation](https://docs.aws.amazon.com/appstream2/latest/APIReference/API_AccessEndpoint.html) for valid values.
    #[builder(into)]
    #[serde(rename = "endpointType")]
    pub r#endpoint_type: Box<String>,
    /// ID of the VPC in which the interface endpoint is used.
    #[builder(into, default)]
    #[serde(rename = "vpceId")]
    pub r#vpce_id: Box<Option<String>>,
}
