#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateTemplate {
    /// Holds the single container that defines the unit of execution for this task.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Option<Vec<super::super::types::cloudrunv2::JobTemplateTemplateContainer>>>,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek
    #[builder(into, default)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Box<Option<String>>,
    /// The execution environment being used to host this Task.
    /// Possible values are: `EXECUTION_ENVIRONMENT_GEN1`, `EXECUTION_ENVIRONMENT_GEN2`.
    #[builder(into, default)]
    #[serde(rename = "executionEnvironment")]
    pub r#execution_environment: Box<Option<String>>,
    /// Number of retries allowed per Task, before marking this Task failed.
    #[builder(into, default)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Box<Option<i32>>,
    /// Email address of the IAM service account associated with the Task of a Job. The service account represents the identity of the running task, and determines what permissions the task has. If not provided, the task will use the project's default service account.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// Max allowed time duration the Task may be active before the system will actively try to mark it failed and kill associated containers. This applies per attempt of a task, meaning each retry can run for the full timeout.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
    /// A list of Volumes to make available to containers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Option<Vec<super::super::types::cloudrunv2::JobTemplateTemplateVolume>>>,
    /// VPC Access configuration to use for this Task. For more information, visit https://cloud.google.com/run/docs/configuring/connecting-vpc.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "vpcAccess")]
    pub r#vpc_access: Box<Option<super::super::types::cloudrunv2::JobTemplateTemplateVpcAccess>>,
}
