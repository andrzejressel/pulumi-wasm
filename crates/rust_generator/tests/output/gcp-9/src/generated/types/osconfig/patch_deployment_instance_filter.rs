#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PatchDeploymentInstanceFilter {
    /// Target all VM instances in the project. If true, no other criteria is permitted.
    #[builder(into, default)]
    #[serde(rename = "all")]
    pub r#all: Box<Option<bool>>,
    /// Targets VM instances matching ANY of these GroupLabels. This allows targeting of disparate groups of VM instances.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "groupLabels")]
    pub r#group_labels: Box<Option<Vec<super::super::types::osconfig::PatchDeploymentInstanceFilterGroupLabel>>>,
    /// Targets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group
    /// VMs when targeting configs, for example prefix="prod-".
    #[builder(into, default)]
    #[serde(rename = "instanceNamePrefixes")]
    pub r#instance_name_prefixes: Box<Option<Vec<String>>>,
    /// Targets any of the VM instances specified. Instances are specified by their URI in the `form zones/{{zone}}/instances/{{instance_name}}`,
    /// `projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}`, or
    /// `https://www.googleapis.com/compute/v1/projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}`
    #[builder(into, default)]
    #[serde(rename = "instances")]
    pub r#instances: Box<Option<Vec<String>>>,
    /// Targets VM instances in ANY of these zones. Leave empty to target VM instances in any zone.
    #[builder(into, default)]
    #[serde(rename = "zones")]
    pub r#zones: Box<Option<Vec<String>>>,
}
