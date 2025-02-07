#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEnvironmentConfigWorkloadsConfig {
    /// Configuration for resources used by DAG processor.
    #[builder(into)]
    #[serde(rename = "dagProcessors")]
    pub r#dag_processors: Box<Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigDagProcessor>>,
    /// Configuration for resources used by Airflow schedulers.
    #[builder(into)]
    #[serde(rename = "schedulers")]
    pub r#schedulers: Box<Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigScheduler>>,
    /// Configuration for resources used by Airflow triggerers.
    #[builder(into)]
    #[serde(rename = "triggerers")]
    pub r#triggerers: Box<Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigTriggerer>>,
    /// Configuration for resources used by Airflow web server.
    #[builder(into)]
    #[serde(rename = "webServers")]
    pub r#web_servers: Box<Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigWebServer>>,
    /// Configuration for resources used by Airflow workers.
    #[builder(into)]
    #[serde(rename = "workers")]
    pub r#workers: Box<Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigWorker>>,
}
