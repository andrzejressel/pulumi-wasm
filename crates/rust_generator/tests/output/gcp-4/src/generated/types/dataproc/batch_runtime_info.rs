#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BatchRuntimeInfo {
    /// (Output)
    /// Approximate workload resource usage, calculated when the workload completes(see [Dataproc Serverless pricing](https://cloud.google.com/dataproc-serverless/pricing))
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "approximateUsages")]
    pub r#approximate_usages: Box<Option<Vec<super::super::types::dataproc::BatchRuntimeInfoApproximateUsage>>>,
    /// (Output)
    /// Snapshot of current workload resource usage(see [Dataproc Serverless pricing](https://cloud.google.com/dataproc-serverless/pricing))
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "currentUsages")]
    pub r#current_usages: Box<Option<Vec<super::super::types::dataproc::BatchRuntimeInfoCurrentUsage>>>,
    /// (Output)
    /// A URI pointing to the location of the diagnostics tarball.
    #[builder(into, default)]
    #[serde(rename = "diagnosticOutputUri")]
    pub r#diagnostic_output_uri: Box<Option<String>>,
    /// (Output)
    /// Map of remote access endpoints (such as web interfaces and APIs) to their URIs.
    #[builder(into, default)]
    #[serde(rename = "endpoints")]
    pub r#endpoints: Box<Option<std::collections::HashMap<String, String>>>,
    /// (Output)
    /// A URI pointing to the location of the stdout and stderr of the workload.
    #[builder(into, default)]
    #[serde(rename = "outputUri")]
    pub r#output_uri: Box<Option<String>>,
}
