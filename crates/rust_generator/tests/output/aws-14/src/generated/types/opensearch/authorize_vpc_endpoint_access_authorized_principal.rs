#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthorizeVpcEndpointAccessAuthorizedPrincipal {
    /// IAM principal that is allowed to access to the domain.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<String>,
    /// Type of principal.
    #[builder(into)]
    #[serde(rename = "principalType")]
    pub r#principal_type: Box<String>,
}
