#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationInitialCapacityInitialCapacityConfig {
    /// The resource configuration of the initial capacity configuration.
    #[builder(into, default)]
    #[serde(rename = "workerConfiguration")]
    pub r#worker_configuration: Box<Option<super::super::types::emrserverless::ApplicationInitialCapacityInitialCapacityConfigWorkerConfiguration>>,
    /// The number of workers in the initial capacity configuration.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: Box<i32>,
}
