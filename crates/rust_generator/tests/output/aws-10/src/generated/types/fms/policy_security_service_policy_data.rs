#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicySecurityServicePolicyData {
    /// Details about the service that are specific to the service type, in JSON format. For service type `SHIELD_ADVANCED`, this is an empty string. Examples depending on `type` can be found in the [AWS Firewall Manager SecurityServicePolicyData API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_SecurityServicePolicyData.html).
    #[builder(into, default)]
    #[serde(rename = "managedServiceData")]
    pub r#managed_service_data: Box<Option<String>>,
    /// Contains the Network Firewall firewall policy options to configure a centralized deployment model. Documented below.
    #[builder(into, default)]
    #[serde(rename = "policyOption")]
    pub r#policy_option: Box<Option<super::super::types::fms::PolicySecurityServicePolicyDataPolicyOption>>,
    /// The service that the policy is using to protect the resources. For the current list of supported types, please refer to the [AWS Firewall Manager SecurityServicePolicyData API Type Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_SecurityServicePolicyData.html#fms-Type-SecurityServicePolicyData-Type).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
