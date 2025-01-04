#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterIngressApplicationGateway {
    /// The ID of the Application Gateway associated with the ingress controller deployed to this Kubernetes Cluster.
    #[builder(into, default)]
    #[serde(rename = "effectiveGatewayId")]
    pub r#effective_gateway_id: Box<Option<String>>,
    /// The ID of the Application Gateway to integrate with the ingress controller of this Kubernetes Cluster. See [this](https://docs.microsoft.com/azure/application-gateway/tutorial-ingress-controller-add-on-existing) page for further details.
    #[builder(into, default)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: Box<Option<String>>,
    /// The name of the Application Gateway to be used or created in the Nodepool Resource Group, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. See [this](https://docs.microsoft.com/azure/application-gateway/tutorial-ingress-controller-add-on-new) page for further details.
    #[builder(into, default)]
    #[serde(rename = "gatewayName")]
    pub r#gateway_name: Box<Option<String>>,
    /// An `ingress_application_gateway_identity` block is exported. The exported attributes are defined below.
    #[builder(into, default)]
    #[serde(rename = "ingressApplicationGatewayIdentities")]
    pub r#ingress_application_gateway_identities: Box<Option<Vec<super::super::types::containerservice::KubernetesClusterIngressApplicationGatewayIngressApplicationGatewayIdentity>>>,
    /// The subnet CIDR to be used to create an Application Gateway, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. See [this](https://docs.microsoft.com/azure/application-gateway/tutorial-ingress-controller-add-on-new) page for further details.
    #[builder(into, default)]
    #[serde(rename = "subnetCidr")]
    pub r#subnet_cidr: Box<Option<String>>,
    /// The ID of the subnet on which to create an Application Gateway, which in turn will be integrated with the ingress controller of this Kubernetes Cluster. See [this](https://docs.microsoft.com/azure/application-gateway/tutorial-ingress-controller-add-on-new) page for further details.
    /// 
    /// > **Note:** Exactly one of `gateway_id`, `subnet_id` or `subnet_cidr` must be specified.
    /// 
    /// > **Note:** If specifying `ingress_application_gateway` in conjunction with `only_critical_addons_enabled`, the AGIC pod will fail to start. A separate `azure.containerservice.KubernetesClusterNodePool` is required to run the AGIC pod successfully. This is because AGIC is classed as a "non-critical addon".
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
