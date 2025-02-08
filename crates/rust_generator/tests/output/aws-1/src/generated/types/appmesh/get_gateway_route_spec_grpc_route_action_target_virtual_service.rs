#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetGatewayRouteSpecGrpcRouteActionTargetVirtualService {
    #[builder(into)]
    #[serde(rename = "virtualServiceName")]
    pub r#virtual_service_name: Box<String>,
}
