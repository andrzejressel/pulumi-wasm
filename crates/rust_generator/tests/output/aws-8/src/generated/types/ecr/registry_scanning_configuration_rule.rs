#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryScanningConfigurationRule {
    /// One or more repository filter blocks, containing a `filter` (required string filtering repositories, see pattern regex [here](https://docs.aws.amazon.com/AmazonECR/latest/APIReference/API_ScanningRepositoryFilter.html)) and a `filter_type` (required string, currently only `WILDCARD` is supported).
    #[builder(into)]
    #[serde(rename = "repositoryFilters")]
    pub r#repository_filters: Box<Vec<super::super::types::ecr::RegistryScanningConfigurationRuleRepositoryFilter>>,
    /// The frequency that scans are performed at for a private registry. Can be `SCAN_ON_PUSH`, `CONTINUOUS_SCAN`, or `MANUAL`.
    #[builder(into)]
    #[serde(rename = "scanFrequency")]
    pub r#scan_frequency: Box<String>,
}
