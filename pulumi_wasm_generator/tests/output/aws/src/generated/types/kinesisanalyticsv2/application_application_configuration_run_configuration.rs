#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationRunConfiguration {
    /// The restore behavior of a restarting application.
    #[builder(into, default)]
    #[serde(rename = "applicationRestoreConfiguration")]
    pub r#application_restore_configuration: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationRunConfigurationApplicationRestoreConfiguration>>,
    /// The starting parameters for a Flink-based Kinesis Data Analytics application.
    #[builder(into, default)]
    #[serde(rename = "flinkRunConfiguration")]
    pub r#flink_run_configuration: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationRunConfigurationFlinkRunConfiguration>>,
}