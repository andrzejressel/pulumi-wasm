#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterNodeConfigLinuxNodeConfigHugepagesConfig {
    /// Amount of 1G hugepages.
    #[builder(into)]
    #[serde(rename = "hugepageSize1g")]
    pub r#hugepage_size_1_g: Box<i32>,
    /// Amount of 2M hugepages.
    #[builder(into)]
    #[serde(rename = "hugepageSize2m")]
    pub r#hugepage_size_2_m: Box<i32>,
}
