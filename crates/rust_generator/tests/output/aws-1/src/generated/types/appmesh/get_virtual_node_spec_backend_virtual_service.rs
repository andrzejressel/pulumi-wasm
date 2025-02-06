#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecBackendVirtualService {
    #[builder(into)]
    #[serde(rename = "clientPolicies")]
    pub r#client_policies: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicy>>,
    #[builder(into)]
    #[serde(rename = "virtualServiceName")]
    pub r#virtual_service_name: Box<String>,
}
