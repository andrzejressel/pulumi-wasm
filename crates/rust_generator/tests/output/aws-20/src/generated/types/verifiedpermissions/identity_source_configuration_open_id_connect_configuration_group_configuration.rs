#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfiguration {
    /// The token claim that you want Verified Permissions to interpret as group membership. For example, `groups`.
    #[builder(into)]
    #[serde(rename = "groupClaim")]
    pub r#group_claim: Box<String>,
    /// The name of the schema entity type that's mapped to the user pool group. Defaults to `AWS::CognitoGroup`.
    #[builder(into)]
    #[serde(rename = "groupEntityType")]
    pub r#group_entity_type: Box<String>,
}
