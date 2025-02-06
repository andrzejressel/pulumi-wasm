#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceExec {
    /// What to run to bring this resource into the desired
    /// state. An exit code of 100 indicates "success", any other exit code
    /// indicates a failure running enforce. Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "enforce")]
    pub r#enforce: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceExecEnforce>>,
    /// What to run to validate this resource is in the
    /// desired state. An exit code of 100 indicates "in desired state", and exit
    /// code of 101 indicates "not in desired state". Any other exit code indicates
    /// a failure running validate. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "validate")]
    pub r#validate: Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidate>,
}
