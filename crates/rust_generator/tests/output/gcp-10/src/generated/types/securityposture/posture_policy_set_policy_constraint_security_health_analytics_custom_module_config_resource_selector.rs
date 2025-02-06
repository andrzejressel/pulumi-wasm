#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModuleConfigResourceSelector {
    /// The resource types to run the detector on.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Vec<String>>,
}
