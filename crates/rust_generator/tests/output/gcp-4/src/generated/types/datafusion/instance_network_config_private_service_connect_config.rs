#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceNetworkConfigPrivateServiceConnectConfig {
    /// (Output)
    /// Output only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.
    /// The size of this block is /25. The format of this field is governed by RFC 4632.
    #[builder(into, default)]
    #[serde(rename = "effectiveUnreachableCidrBlock")]
    pub r#effective_unreachable_cidr_block: Box<Option<String>>,
    /// Optional. The reference to the network attachment used to establish private connectivity.
    /// It will be of the form projects/{project-id}/regions/{region}/networkAttachments/{network-attachment-id}.
    /// This is required only when using connection type PRIVATE_SERVICE_CONNECT_INTERFACES.
    #[builder(into, default)]
    #[serde(rename = "networkAttachment")]
    pub r#network_attachment: Box<Option<String>>,
    /// Optional. Input only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.
    /// The size of this block should be at least /25. This range should not overlap with the primary address range of any subnetwork used by the network attachment.
    /// This range can be used for other purposes in the consumer VPC as long as there is no requirement for CDF to reach destinations using these addresses.
    /// If this value is not provided, the server chooses a non RFC 1918 address range. The format of this field is governed by RFC 4632.
    #[builder(into, default)]
    #[serde(rename = "unreachableCidrBlock")]
    pub r#unreachable_cidr_block: Box<Option<String>>,
}
