#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScaleSetPacketCaptureMachineScope {
    /// A list of Virtual Machine Scale Set instance IDs which should be excluded from running Packet Capture, e.g. `["0", "2"]`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "excludeInstanceIds")]
    pub r#exclude_instance_ids: Box<Option<Vec<String>>>,
    /// A list of Virtual Machine Scale Set instance IDs which should be included for Packet Capture, e.g. `["1", "3"]`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "includeInstanceIds")]
    pub r#include_instance_ids: Box<Option<Vec<String>>>,
}
