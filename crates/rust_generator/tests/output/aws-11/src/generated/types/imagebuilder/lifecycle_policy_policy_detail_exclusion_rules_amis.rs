#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailExclusionRulesAmis {
    /// Configures whether public AMIs are excluded from the lifecycle action.
    #[builder(into, default)]
    #[serde(rename = "isPublic")]
    pub r#is_public: Box<Option<bool>>,
    /// Specifies configuration details for Image Builder to exclude the most recent resources from lifecycle actions. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "lastLaunched")]
    pub r#last_launched: Box<Option<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRulesAmisLastLaunched>>,
    /// Configures AWS Regions that are excluded from the lifecycle action.
    #[builder(into, default)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Option<Vec<String>>>,
    /// Specifies AWS accounts whose resources are excluded from the lifecycle action.
    #[builder(into, default)]
    #[serde(rename = "sharedAccounts")]
    pub r#shared_accounts: Box<Option<Vec<String>>>,
    /// Lists tags that should be excluded from lifecycle actions for the AMIs that have them.
    #[builder(into, default)]
    #[serde(rename = "tagMap")]
    pub r#tag_map: Box<Option<std::collections::HashMap<String, String>>>,
}
