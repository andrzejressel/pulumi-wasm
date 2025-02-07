#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupInventoryFilter {
    /// The OS short name
    #[builder(into)]
    #[serde(rename = "osShortName")]
    pub r#os_short_name: Box<String>,
    /// The OS version Prefix matches are supported if
    /// asterisk(*) is provided as the last character. For example, to match all
    /// versions with a major version of `7`, specify the following value for this
    /// field `7.*` An empty string matches all OS versions.
    #[builder(into, default)]
    #[serde(rename = "osVersion")]
    pub r#os_version: Box<Option<String>>,
}
