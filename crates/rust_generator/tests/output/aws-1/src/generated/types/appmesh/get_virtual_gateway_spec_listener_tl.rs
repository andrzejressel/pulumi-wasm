#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualGatewaySpecListenerTl {
    #[builder(into)]
    #[serde(rename = "certificates")]
    pub r#certificates: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerTlCertificate>>,
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    #[builder(into)]
    #[serde(rename = "validations")]
    pub r#validations: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerTlValidation>>,
}
