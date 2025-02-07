#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServerlessCacheCacheUsageLimits {
    /// The maximum data storage limit in the cache, expressed in Gigabytes. See `data_storage` Block for details.
    #[builder(into)]
    #[serde(rename = "dataStorage")]
    pub r#data_storage: Box<super::super::types::elasticache::GetServerlessCacheCacheUsageLimitsDataStorage>,
    /// The configured number of ElastiCache Processing Units (ECPU) the cache can consume per second. See `ecpu_per_second` Block for details.
    #[builder(into)]
    #[serde(rename = "ecpuPerSecond")]
    pub r#ecpu_per_second: Box<super::super::types::elasticache::GetServerlessCacheCacheUsageLimitsEcpuPerSecond>,
}
