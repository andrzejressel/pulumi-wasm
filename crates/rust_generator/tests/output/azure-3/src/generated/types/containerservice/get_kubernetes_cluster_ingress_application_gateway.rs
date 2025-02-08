#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetKubernetesClusterIngressApplicationGateway {
    /// The ID of the Application Gateway associated with the ingress controller deployed to this Kubernetes Cluster.
    #[builder(into)]
    #[serde(rename = "effectiveGatewayId")]
    pub r#effective_gateway_id: Box<String>,
    /// The ID of the Application Gateway integrated with the ingress controller of this Kubernetes Cluster. This attribute is only set when gateway_id is specified when configuring the `ingress_application_gateway` addon.
    #[builder(into)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "gatewayName")]
    pub r#gateway_name: Box<String>,
    /// An `ingress_application_gateway_identity` block as defined below.
    #[builder(into)]
    #[serde(rename = "ingressApplicationGatewayIdentities")]
    pub r#ingress_application_gateway_identities: Box<Vec<super::super::types::containerservice::GetKubernetesClusterIngressApplicationGatewayIngressApplicationGatewayIdentity>>,
    /// The subnet CIDR used to create an Application Gateway, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. This attribute is only set when `subnet_cidr` is specified when configuring the `ingress_application_gateway` addon.
    #[builder(into)]
    #[serde(rename = "subnetCidr")]
    pub r#subnet_cidr: Box<String>,
    /// The ID of the subnet on which to create an Application Gateway, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. This attribute is only set when `subnet_id` is specified when configuring the `ingress_application_gateway` addon.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
