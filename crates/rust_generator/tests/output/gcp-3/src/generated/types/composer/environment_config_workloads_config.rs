#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentConfigWorkloadsConfig {
    /// Configuration for resources used by DAG processor.
    #[builder(into, default)]
    #[serde(rename = "dagProcessor")]
    pub r#dag_processor: Box<Option<super::super::types::composer::EnvironmentConfigWorkloadsConfigDagProcessor>>,
    /// Configuration for resources used by Airflow schedulers.
    #[builder(into, default)]
    #[serde(rename = "scheduler")]
    pub r#scheduler: Box<Option<super::super::types::composer::EnvironmentConfigWorkloadsConfigScheduler>>,
    /// Configuration for resources used by Airflow triggerers.
    #[builder(into, default)]
    #[serde(rename = "triggerer")]
    pub r#triggerer: Box<Option<super::super::types::composer::EnvironmentConfigWorkloadsConfigTriggerer>>,
    /// Configuration for resources used by Airflow web server.
    #[builder(into, default)]
    #[serde(rename = "webServer")]
    pub r#web_server: Box<Option<super::super::types::composer::EnvironmentConfigWorkloadsConfigWebServer>>,
    /// Configuration for resources used by Airflow workers.
    #[builder(into, default)]
    #[serde(rename = "worker")]
    pub r#worker: Box<Option<super::super::types::composer::EnvironmentConfigWorkloadsConfigWorker>>,
}
