#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterWebAppRouting {
    /// Specifies the list of the DNS Zone IDs in which DNS entries are created for applications deployed to the cluster when Web App Routing is enabled. If not using Bring-Your-Own DNS zones this property should be set to an empty list.
    #[builder(into)]
    #[serde(rename = "dnsZoneIds")]
    pub r#dns_zone_ids: Box<Vec<String>>,
    /// A `web_app_routing_identity` block is exported. The exported attributes are defined below.
    #[builder(into, default)]
    #[serde(rename = "webAppRoutingIdentities")]
    pub r#web_app_routing_identities: Box<Option<Vec<super::super::types::containerservice::KubernetesClusterWebAppRoutingWebAppRoutingIdentity>>>,
}
