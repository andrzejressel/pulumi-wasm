#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstancePerformanceConfigFixedIop {
    /// The number of IOPS to provision for the instance.
    /// max_iops must be in multiple of 1000.
    #[builder(into)]
    #[serde(rename = "maxIops")]
    pub r#max_iops: Box<i32>,
}
