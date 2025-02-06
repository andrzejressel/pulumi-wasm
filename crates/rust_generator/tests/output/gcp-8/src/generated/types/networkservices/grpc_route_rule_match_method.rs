#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GrpcRouteRuleMatchMethod {
    /// Specifies that matches are case sensitive. The default value is true.
    #[builder(into, default)]
    #[serde(rename = "caseSensitive")]
    pub r#case_sensitive: Box<Option<bool>>,
    /// Required. Name of the method to match against.
    #[builder(into)]
    #[serde(rename = "grpcMethod")]
    pub r#grpc_method: Box<String>,
    /// Required. Name of the service to match against.
    #[builder(into)]
    #[serde(rename = "grpcService")]
    pub r#grpc_service: Box<String>,
}
