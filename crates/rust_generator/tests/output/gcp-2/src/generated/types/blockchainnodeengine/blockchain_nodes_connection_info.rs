#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BlockchainNodesConnectionInfo {
    /// (Output)
    /// The endpoint information through which to interact with a blockchain node.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "endpointInfos")]
    pub r#endpoint_infos: Box<Option<Vec<super::super::types::blockchainnodeengine::BlockchainNodesConnectionInfoEndpointInfo>>>,
    /// (Output)
    /// A service attachment that exposes a node, and has the following format: projects/{project}/regions/{region}/serviceAttachments/{service_attachment_name}
    #[builder(into, default)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Box<Option<String>>,
}
