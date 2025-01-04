#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplateJob {
    /// Job is a Hadoop job.
    #[builder(into, default)]
    #[serde(rename = "hadoopJob")]
    pub r#hadoop_job: Box<Option<super::super::types::dataproc::WorkflowTemplateJobHadoopJob>>,
    /// Job is a Hive job.
    #[builder(into, default)]
    #[serde(rename = "hiveJob")]
    pub r#hive_job: Box<Option<super::super::types::dataproc::WorkflowTemplateJobHiveJob>>,
    /// The labels to associate with this job. Label keys must be between 1 and 63 characters long, and must conform to the following regular expression: {0,63} No more than 32 labels can be associated with a given job.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Job is a Pig job.
    #[builder(into, default)]
    #[serde(rename = "pigJob")]
    pub r#pig_job: Box<Option<super::super::types::dataproc::WorkflowTemplateJobPigJob>>,
    /// The optional list of prerequisite job step_ids. If not specified, the job will start at the beginning of workflow.
    #[builder(into, default)]
    #[serde(rename = "prerequisiteStepIds")]
    pub r#prerequisite_step_ids: Box<Option<Vec<String>>>,
    /// Job is a Presto job.
    #[builder(into, default)]
    #[serde(rename = "prestoJob")]
    pub r#presto_job: Box<Option<super::super::types::dataproc::WorkflowTemplateJobPrestoJob>>,
    /// Job is a PySpark job.
    #[builder(into, default)]
    #[serde(rename = "pysparkJob")]
    pub r#pyspark_job: Box<Option<super::super::types::dataproc::WorkflowTemplateJobPysparkJob>>,
    /// Job scheduling configuration.
    #[builder(into, default)]
    #[serde(rename = "scheduling")]
    pub r#scheduling: Box<Option<super::super::types::dataproc::WorkflowTemplateJobScheduling>>,
    /// Job is a Spark job.
    #[builder(into, default)]
    #[serde(rename = "sparkJob")]
    pub r#spark_job: Box<Option<super::super::types::dataproc::WorkflowTemplateJobSparkJob>>,
    /// Job is a SparkR job.
    #[builder(into, default)]
    #[serde(rename = "sparkRJob")]
    pub r#spark_r_job: Box<Option<super::super::types::dataproc::WorkflowTemplateJobSparkRJob>>,
    /// Job is a SparkSql job.
    #[builder(into, default)]
    #[serde(rename = "sparkSqlJob")]
    pub r#spark_sql_job: Box<Option<super::super::types::dataproc::WorkflowTemplateJobSparkSqlJob>>,
    /// Required. The step id. The id must be unique among all jobs within the template. The step id is used as prefix for job id, as job `goog-dataproc-workflow-step-id` label, and in field from other steps. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters.
    #[builder(into)]
    #[serde(rename = "stepId")]
    pub r#step_id: Box<String>,
}
