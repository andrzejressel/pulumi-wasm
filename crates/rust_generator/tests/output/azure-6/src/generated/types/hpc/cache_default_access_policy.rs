#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CacheDefaultAccessPolicy {
    /// One or more `access_rule` blocks (up to three) as defined above.
    #[builder(into)]
    #[serde(rename = "accessRules")]
    pub r#access_rules: Box<Vec<super::super::types::hpc::CacheDefaultAccessPolicyAccessRule>>,
}
