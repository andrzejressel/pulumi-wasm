#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxVirtualMachineInboundNatRule {
    /// The Backend Port associated with this NAT Rule. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: Box<i32>,
    /// The frontend port associated with this Inbound NAT Rule.
    #[builder(into, default)]
    #[serde(rename = "frontendPort")]
    pub r#frontend_port: Box<Option<i32>>,
    /// The Protocol used for this NAT Rule. Possible values are `Tcp` and `Udp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}
