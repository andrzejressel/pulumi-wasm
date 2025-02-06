#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuestPoliciesAssignment {
    /// Targets instances matching at least one of these label sets. This allows an assignment to target disparate groups,
    /// for example "env=prod or env=staging".
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "groupLabels")]
    pub r#group_labels: Box<Option<Vec<super::super::types::osconfig::GuestPoliciesAssignmentGroupLabel>>>,
    /// Targets VM instances whose name starts with one of these prefixes.
    /// Like labels, this is another way to group VM instances when targeting configs,
    /// for example prefix="prod-".
    /// Only supported for project-level policies.
    #[builder(into, default)]
    #[serde(rename = "instanceNamePrefixes")]
    pub r#instance_name_prefixes: Box<Option<Vec<String>>>,
    /// Targets any of the instances specified. Instances are specified by their URI in the form
    /// zones/[ZONE]/instances/[INSTANCE_NAME].
    /// Instance targeting is uncommon and is supported to facilitate the management of changes
    /// by the instance or to target specific VM instances for development and testing.
    /// Only supported for project-level policies and must reference instances within this project.
    #[builder(into, default)]
    #[serde(rename = "instances")]
    pub r#instances: Box<Option<Vec<String>>>,
    /// Targets VM instances matching at least one of the following OS types.
    /// VM instances must match all supplied criteria for a given OsType to be included.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "osTypes")]
    pub r#os_types: Box<Option<Vec<super::super::types::osconfig::GuestPoliciesAssignmentOsType>>>,
    /// Targets instances in any of these zones. Leave empty to target instances in any zone.
    /// Zonal targeting is uncommon and is supported to facilitate the management of changes by zone.
    #[builder(into, default)]
    #[serde(rename = "zones")]
    pub r#zones: Box<Option<Vec<String>>>,
}
