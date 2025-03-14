#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedZoneForwardingConfig {
    /// List of target name servers to forward to. Cloud DNS will
    /// select the best available name server if more than
    /// one target is given.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "targetNameServers")]
    pub r#target_name_servers: Box<Vec<super::super::types::dns::ManagedZoneForwardingConfigTargetNameServer>>,
}
