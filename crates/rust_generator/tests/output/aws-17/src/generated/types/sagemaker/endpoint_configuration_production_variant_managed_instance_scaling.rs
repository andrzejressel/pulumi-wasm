#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EndpointConfigurationProductionVariantManagedInstanceScaling {
    /// The maximum number of instances that the endpoint can provision when it scales up to accommodate an increase in traffic.
    #[builder(into, default)]
    #[serde(rename = "maxInstanceCount")]
    pub r#max_instance_count: Box<Option<i32>>,
    /// The minimum number of instances that the endpoint must retain when it scales down to accommodate a decrease in traffic.
    #[builder(into, default)]
    #[serde(rename = "minInstanceCount")]
    pub r#min_instance_count: Box<Option<i32>>,
    /// Indicates whether managed instance scaling is enabled. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
