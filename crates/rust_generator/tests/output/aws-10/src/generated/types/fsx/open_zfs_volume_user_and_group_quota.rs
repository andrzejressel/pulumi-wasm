#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OpenZfsVolumeUserAndGroupQuota {
    /// The ID of the user or group. Valid values between `0` and `2147483647`
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<i32>,
    /// The amount of storage that the user or group can use in gibibytes (GiB). Valid values between `0` and `2147483647`
    /// * `Type` - (Required) - A value that specifies whether the quota applies to a user or group. Valid values are `USER` or `GROUP`.
    #[builder(into)]
    #[serde(rename = "storageCapacityQuotaGib")]
    pub r#storage_capacity_quota_gib: Box<i32>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
