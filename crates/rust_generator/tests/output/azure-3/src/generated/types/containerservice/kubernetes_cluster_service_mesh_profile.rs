#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterServiceMeshProfile {
    /// A `certificate_authority` block as defined below. When this property is specified, `key_vault_secrets_provider` is also required to be set. This configuration allows you to bring your own root certificate and keys for Istio CA in the Istio-based service mesh add-on for Azure Kubernetes Service.
    #[builder(into, default)]
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Box<Option<super::super::types::containerservice::KubernetesClusterServiceMeshProfileCertificateAuthority>>,
    /// Is Istio External Ingress Gateway enabled?
    /// 
    /// > **NOTE:** Currently only one Internal Ingress Gateway and one External Ingress Gateway are allowed per cluster
    #[builder(into, default)]
    #[serde(rename = "externalIngressGatewayEnabled")]
    pub r#external_ingress_gateway_enabled: Box<Option<bool>>,
    /// Is Istio Internal Ingress Gateway enabled?
    #[builder(into, default)]
    #[serde(rename = "internalIngressGatewayEnabled")]
    pub r#internal_ingress_gateway_enabled: Box<Option<bool>>,
    /// The mode of the service mesh. Possible value is `Istio`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Specify 1 or 2 Istio control plane revisions for managing minor upgrades using the canary upgrade process. For example, create the resource with `revisions` set to `["asm-1-20"]`, or leave it empty (the `revisions` will only be known after apply). To start the canary upgrade, change `revisions` to `["asm-1-20", "asm-1-21"]`. To roll back the canary upgrade, revert to `["asm-1-20"]`. To confirm the upgrade, change to `["asm-1-21"]`.
    /// 
    /// > **NOTE:** Upgrading to a new (canary) revision does not affect existing sidecar proxies. You need to apply the canary revision label to selected namespaces and restart pods with kubectl to inject the new sidecar proxy. [Learn more](https://istio.io/latest/docs/setup/upgrade/canary/#data-plane).
    #[builder(into)]
    #[serde(rename = "revisions")]
    pub r#revisions: Box<Vec<String>>,
}
