#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CrossAccountAttachmentResource {
    /// IP address range, in CIDR format, that is specified as resource.
    #[builder(into, default)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<Option<String>>,
    /// The endpoint ID for the endpoint that is specified as a AWS resource.
    #[builder(into, default)]
    #[serde(rename = "endpointId")]
    pub r#endpoint_id: Box<Option<String>>,
    /// The AWS Region where a shared endpoint resource is located.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
}
