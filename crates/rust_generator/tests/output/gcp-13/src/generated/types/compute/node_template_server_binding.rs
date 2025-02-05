#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodeTemplateServerBinding {
    /// Type of server binding policy. If `RESTART_NODE_ON_ANY_SERVER`,
    /// nodes using this template will restart on any physical server
    /// following a maintenance event.
    /// If `RESTART_NODE_ON_MINIMAL_SERVER`, nodes using this template
    /// will restart on the same physical server following a maintenance
    /// event, instead of being live migrated to or restarted on a new
    /// physical server. This option may be useful if you are using
    /// software licenses tied to the underlying server characteristics
    /// such as physical sockets or cores, to avoid the need for
    /// additional licenses when maintenance occurs. However, VMs on such
    /// nodes will experience outages while maintenance is applied.
    /// Possible values are: `RESTART_NODE_ON_ANY_SERVER`, `RESTART_NODE_ON_MINIMAL_SERVERS`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
