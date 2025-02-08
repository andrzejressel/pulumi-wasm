#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SpotInstanceRequestNetworkInterface {
    /// Whether or not to delete the network interface on instance termination. Defaults to `false`. Currently, the only valid value is `false`, as this is only supported when creating new network interfaces when launching an instance.
    #[builder(into, default)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Box<Option<bool>>,
    /// Integer index of the network interface attachment. Limited by instance type.
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: Box<i32>,
    /// Integer index of the network card. Limited by instance type. The default index is `0`.
    #[builder(into, default)]
    #[serde(rename = "networkCardIndex")]
    pub r#network_card_index: Box<Option<i32>>,
    /// ID of the network interface to attach.
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Box<String>,
}
