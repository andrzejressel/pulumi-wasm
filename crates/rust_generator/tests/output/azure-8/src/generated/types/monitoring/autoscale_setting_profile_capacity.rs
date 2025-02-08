#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutoscaleSettingProfileCapacity {
    /// The number of instances that are available for scaling if metrics are not available for evaluation. The default is only used if the current instance count is lower than the default. Valid values are between `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "default")]
    pub r#default: Box<i32>,
    /// The maximum number of instances for this resource. Valid values are between `0` and `1000`.
    /// 
    /// > **NOTE:** The maximum number of instances is also limited by the amount of Cores available in the subscription.
    #[builder(into)]
    #[serde(rename = "maximum")]
    pub r#maximum: Box<i32>,
    /// The minimum number of instances for this resource. Valid values are between `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "minimum")]
    pub r#minimum: Box<i32>,
}
