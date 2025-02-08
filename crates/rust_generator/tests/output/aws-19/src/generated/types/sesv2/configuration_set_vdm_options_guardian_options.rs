#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationSetVdmOptionsGuardianOptions {
    /// Specifies the status of your VDM optimized shared delivery. Valid values: `ENABLED`, `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "optimizedSharedDelivery")]
    pub r#optimized_shared_delivery: Box<Option<String>>,
}
