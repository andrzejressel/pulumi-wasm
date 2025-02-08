#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RiskConfigurationCompromisedCredentialsRiskConfiguration {
    /// The compromised credentials risk configuration actions. See details below.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<super::super::types::cognito::RiskConfigurationCompromisedCredentialsRiskConfigurationActions>,
    /// Perform the action for these events. The default is to perform all events if no event filter is specified. Valid values are `SIGN_IN`, `PASSWORD_CHANGE`, and `SIGN_UP`.
    #[builder(into, default)]
    #[serde(rename = "eventFilters")]
    pub r#event_filters: Box<Option<Vec<String>>>,
}
