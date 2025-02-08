#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegionSecurityPolicyDdosProtectionConfig {
    /// Google Cloud Armor offers the following options to help protect systems against DDoS attacks:
    /// - STANDARD: basic always-on protection for network load balancers, protocol forwarding, or VMs with public IP addresses.
    /// - ADVANCED: additional protections for Managed Protection Plus subscribers who use network load balancers, protocol forwarding, or VMs with public IP addresses.
    /// - ADVANCED_PREVIEW: flag to enable the security policy in preview mode.
    /// Possible values are: `ADVANCED`, `ADVANCED_PREVIEW`, `STANDARD`.
    #[builder(into)]
    #[serde(rename = "ddosProtection")]
    pub r#ddos_protection: Box<String>,
}
