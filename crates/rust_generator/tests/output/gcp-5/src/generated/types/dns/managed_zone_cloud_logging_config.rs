#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ManagedZoneCloudLoggingConfig {
    /// If set, enable query logging for this ManagedZone. False by default, making logging opt-in.
    #[builder(into)]
    #[serde(rename = "enableLogging")]
    pub r#enable_logging: Box<bool>,
}
