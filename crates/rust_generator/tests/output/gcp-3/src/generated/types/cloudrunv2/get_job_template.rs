#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobTemplate {
    /// Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects.
    /// 
    /// Cloud Run API v2 does not support annotations with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.
    /// All system annotations in v1 now have a corresponding field in v2 ExecutionTemplate.
    /// 
    /// This field follows Kubernetes annotations' namespacing, limits, and rules.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: Box<std::collections::HashMap<String, String>>,
    /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter,
    /// or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or
    /// https://cloud.google.com/run/docs/configuring/labels.
    /// 
    /// Cloud Run API v2 does not support labels with 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected.
    /// All system labels in v1 now have a corresponding field in v2 ExecutionTemplate.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// Specifies the maximum desired number of tasks the execution should run at given time. Must be <= taskCount. When the job is run, if this field is 0 or unset, the maximum possible value will be used for that execution. The actual number of tasks running in steady state will be less than this number when there are fewer tasks waiting to be completed remaining, i.e. when the work left to do is less than max parallelism.
    #[builder(into)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<i32>,
    /// Specifies the desired number of tasks the execution should run. Setting to 1 means that parallelism is limited to 1 and the success of that task signals the success of the execution. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[builder(into)]
    #[serde(rename = "taskCount")]
    pub r#task_count: Box<i32>,
    /// Describes the task(s) that will be created when executing an execution
    #[builder(into)]
    #[serde(rename = "templates")]
    pub r#templates: Box<Vec<super::super::types::cloudrunv2::GetJobTemplateTemplate>>,
}
