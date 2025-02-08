#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceGroupManagerStatefulExternalIp {
    /// , A value that prescribes what should happen to the external ip when the VM instance is deleted. The available options are `NEVER` and `ON_PERMANENT_INSTANCE_DELETION`. `NEVER` - detach the ip when the VM is deleted, but do not delete the ip. `ON_PERMANENT_INSTANCE_DELETION` will delete the external ip when the VM is permanently deleted from the instance group.
    #[builder(into, default)]
    #[serde(rename = "deleteRule")]
    pub r#delete_rule: Box<Option<String>>,
    /// , The network interface name of the external Ip. Possible value: `nic0`
    #[builder(into, default)]
    #[serde(rename = "interfaceName")]
    pub r#interface_name: Box<Option<String>>,
}
