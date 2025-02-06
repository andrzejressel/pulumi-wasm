#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentOsPolicy {
    /// This flag determines the OS
    /// policy compliance status when none of the resource groups within the policy
    /// are applicable for a VM. Set this value to `true` if the policy needs to be
    /// reported as compliant even if the policy has nothing to validate or enforce.
    #[builder(into, default)]
    #[serde(rename = "allowNoResourceGroupMatch")]
    pub r#allow_no_resource_group_match: Box<Option<bool>>,
    /// Policy description. Length of the description is
    /// limited to 1024 characters.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The id of the OS policy with the following restrictions:
    /// 
    /// *   Must contain only lowercase letters, numbers, and hyphens.
    /// *   Must start with a letter.
    /// *   Must be between 1-63 characters.
    /// *   Must end with a number or a letter.
    /// *   Must be unique within the assignment.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Policy mode Possible values are: `MODE_UNSPECIFIED`,
    /// `VALIDATION`, `ENFORCEMENT`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// List of resource groups for the policy. For a
    /// particular VM, resource groups are evaluated in the order specified and the
    /// first resource group that is applicable is selected and the rest are
    /// ignored. If none of the resource groups are applicable for a VM, the VM is
    /// considered to be non-compliant w.r.t this policy. This behavior can be
    /// toggled by the flag `allow_no_resource_group_match` Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "resourceGroups")]
    pub r#resource_groups: Box<Vec<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroup>>,
}
