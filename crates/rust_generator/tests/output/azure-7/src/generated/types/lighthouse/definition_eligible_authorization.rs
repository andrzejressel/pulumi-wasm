#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DefinitionEligibleAuthorization {
    /// A `just_in_time_access_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "justInTimeAccessPolicy")]
    pub r#just_in_time_access_policy: Box<Option<super::super::types::lighthouse::DefinitionEligibleAuthorizationJustInTimeAccessPolicy>>,
    /// The display name of the Azure Active Directory Principal.
    #[builder(into, default)]
    #[serde(rename = "principalDisplayName")]
    pub r#principal_display_name: Box<Option<String>>,
    /// Principal ID of the security group/service principal/user that would be assigned permissions to the projected subscription.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Principal ID of the Azure built-in role that defines the permissions that the Azure Active Directory will have on the projected scope.
    #[builder(into)]
    #[serde(rename = "roleDefinitionId")]
    pub r#role_definition_id: Box<String>,
}
