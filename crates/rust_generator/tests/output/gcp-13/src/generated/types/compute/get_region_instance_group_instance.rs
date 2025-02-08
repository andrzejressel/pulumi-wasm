#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRegionInstanceGroupInstance {
    /// URL to the instance.
    #[builder(into)]
    #[serde(rename = "instance")]
    pub r#instance: Box<String>,
    /// List of named ports in the group, as a list of resources, each containing:
    #[builder(into)]
    #[serde(rename = "namedPorts")]
    pub r#named_ports: Box<Vec<super::super::types::compute::GetRegionInstanceGroupInstanceNamedPort>>,
    /// String description of current state of the instance.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
