#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceGceSetupNetworkInterfaceAccessConfig {
    /// An external IP address associated with this instance. Specify an unused
    /// static external IP address available to the project or leave this field
    /// undefined to use an IP from a shared ephemeral IP address pool. If you
    /// specify a static external IP address, it must live in the same region as
    /// the zone of the instance.
    #[builder(into)]
    #[serde(rename = "externalIp")]
    pub r#external_ip: Box<String>,
}
