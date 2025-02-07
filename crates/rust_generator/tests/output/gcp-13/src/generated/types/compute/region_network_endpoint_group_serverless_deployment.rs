#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionNetworkEndpointGroupServerlessDeployment {
    /// The platform of the NEG backend target(s). Possible values:
    /// API Gateway: apigateway.googleapis.com
    #[builder(into)]
    #[serde(rename = "platform")]
    pub r#platform: Box<String>,
    /// The user-defined name of the workload/instance. This value must be provided explicitly or in the urlMask.
    /// The resource identified by this value is platform-specific and is as follows: API Gateway: The gateway ID, App Engine: The service name,
    /// Cloud Functions: The function name, Cloud Run: The service name
    #[builder(into, default)]
    #[serde(rename = "resource")]
    pub r#resource: Box<Option<String>>,
    /// A template to parse platform-specific fields from a request URL. URL mask allows for routing to multiple resources
    /// on the same serverless platform without having to create multiple Network Endpoint Groups and backend resources.
    /// The fields parsed by this template are platform-specific and are as follows: API Gateway: The gateway ID,
    /// App Engine: The service and version, Cloud Functions: The function name, Cloud Run: The service and tag
    #[builder(into, default)]
    #[serde(rename = "urlMask")]
    pub r#url_mask: Box<Option<String>>,
    /// The optional resource version. The version identified by this value is platform-specific and is follows:
    /// API Gateway: Unused, App Engine: The service version, Cloud Functions: Unused, Cloud Run: The service tag
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
