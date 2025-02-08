#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfiguration {
    /// The name of the schema entity type that's mapped to the user pool group. Defaults to `AWS::CognitoGroup`.
    #[builder(into)]
    #[serde(rename = "groupEntityType")]
    pub r#group_entity_type: Box<String>,
}
