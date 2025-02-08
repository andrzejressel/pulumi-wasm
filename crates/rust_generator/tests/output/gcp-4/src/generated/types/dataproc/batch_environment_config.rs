#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BatchEnvironmentConfig {
    /// Execution configuration for a workload.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "executionConfig")]
    pub r#execution_config: Box<Option<super::super::types::dataproc::BatchEnvironmentConfigExecutionConfig>>,
    /// Peripherals configuration that workload has access to.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "peripheralsConfig")]
    pub r#peripherals_config: Box<Option<super::super::types::dataproc::BatchEnvironmentConfigPeripheralsConfig>>,
}
