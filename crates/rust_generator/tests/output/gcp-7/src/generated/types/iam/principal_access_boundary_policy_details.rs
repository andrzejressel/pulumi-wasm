#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PrincipalAccessBoundaryPolicyDetails {
    /// The version number that indicates which Google Cloud services
    /// are included in the enforcement (e.g. \"latest\", \"1\", ...). If empty, the
    /// PAB policy version will be set to the current latest version, and this version
    /// won't get updated when new versions are released.
    #[builder(into, default)]
    #[serde(rename = "enforcementVersion")]
    pub r#enforcement_version: Box<Option<String>>,
    /// A list of principal access boundary policy rules. The number of rules in a policy is limited to 500.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::iam::PrincipalAccessBoundaryPolicyDetailsRule>>,
}
