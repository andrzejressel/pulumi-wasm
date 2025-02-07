#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDelegatedServicesDelegatedService {
    /// The date that the account became a delegated administrator for this service.
    #[builder(into)]
    #[serde(rename = "delegationEnabledDate")]
    pub r#delegation_enabled_date: Box<String>,
    /// The name of an AWS service that can request an operation for the specified service.
    #[builder(into)]
    #[serde(rename = "servicePrincipal")]
    pub r#service_principal: Box<String>,
}
