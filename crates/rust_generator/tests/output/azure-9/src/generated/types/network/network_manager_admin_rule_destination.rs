#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkManagerAdminRuleDestination {
    /// Specifies the address prefix.
    #[builder(into)]
    #[serde(rename = "addressPrefix")]
    pub r#address_prefix: Box<String>,
    /// Specifies the address prefix type. Possible values are `IPPrefix` and `ServiceTag`. For more information, please see [this document](https://learn.microsoft.com/en-us/azure/virtual-network-manager/concept-security-admins#source-and-destination-types).
    #[builder(into)]
    #[serde(rename = "addressPrefixType")]
    pub r#address_prefix_type: Box<String>,
}
