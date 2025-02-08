#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTiersTier {
    /// The maximum disk size of this tier in bytes.
    #[builder(into)]
    #[serde(rename = "diskQuota")]
    pub r#disk_quota: Box<i32>,
    /// The maximum ram usage of this tier in bytes.
    #[builder(into)]
    #[serde(rename = "ram")]
    pub r#ram: Box<i32>,
    /// The applicable regions for this tier.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Vec<String>>,
    /// An identifier for the machine type, for example, db-custom-1-3840.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
