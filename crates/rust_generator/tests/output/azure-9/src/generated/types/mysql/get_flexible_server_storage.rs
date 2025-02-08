#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFlexibleServerStorage {
    /// Is Storage Auto Grow enabled?
    #[builder(into)]
    #[serde(rename = "autoGrowEnabled")]
    pub r#auto_grow_enabled: Box<bool>,
    /// Should IOPS be scaled automatically?
    #[builder(into)]
    #[serde(rename = "ioScalingEnabled")]
    pub r#io_scaling_enabled: Box<bool>,
    /// The storage IOPS of the MySQL Flexible Server.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Box<i32>,
    /// The max storage allowed for the MySQL Flexible Server.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Box<i32>,
}
