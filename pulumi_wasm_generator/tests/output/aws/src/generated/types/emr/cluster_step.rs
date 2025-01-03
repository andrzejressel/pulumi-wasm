#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterStep {
    /// Action to take if the step fails. Valid values: `TERMINATE_JOB_FLOW`, `TERMINATE_CLUSTER`, `CANCEL_AND_WAIT`, and `CONTINUE`
    #[builder(into)]
    #[serde(rename = "actionOnFailure")]
    pub r#action_on_failure: Box<String>,
    /// JAR file used for the step. See below.
    #[builder(into)]
    #[serde(rename = "hadoopJarStep")]
    pub r#hadoop_jar_step: Box<super::super::types::emr::ClusterStepHadoopJarStep>,
    /// Name of the step.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
