#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointDeploymentConfigRollingUpdatePolicy {
    /// Batch size for each rolling step to provision capacity and turn on traffic on the new endpoint fleet, and terminate capacity on the old endpoint fleet. Value must be between 5% to 50% of the variant's total instance count. See Maximum Batch Size.
    #[builder(into)]
    #[serde(rename = "maximumBatchSize")]
    pub r#maximum_batch_size: Box<super::super::types::sagemaker::EndpointDeploymentConfigRollingUpdatePolicyMaximumBatchSize>,
    /// The time limit for the total deployment. Exceeding this limit causes a timeout. Valid values are between `600` and `14400`.
    #[builder(into, default)]
    #[serde(rename = "maximumExecutionTimeoutInSeconds")]
    pub r#maximum_execution_timeout_in_seconds: Box<Option<i32>>,
    /// Batch size for rollback to the old endpoint fleet. Each rolling step to provision capacity and turn on traffic on the old endpoint fleet, and terminate capacity on the new endpoint fleet. If this field is absent, the default value will be set to 100% of total capacity which means to bring up the whole capacity of the old fleet at once during rollback. See Rollback Maximum Batch Size.
    #[builder(into, default)]
    #[serde(rename = "rollbackMaximumBatchSize")]
    pub r#rollback_maximum_batch_size: Box<Option<super::super::types::sagemaker::EndpointDeploymentConfigRollingUpdatePolicyRollbackMaximumBatchSize>>,
    /// The length of the baking period, during which SageMaker monitors alarms for each batch on the new fleet. Valid values are between `0` and `3600`.
    #[builder(into)]
    #[serde(rename = "waitIntervalInSeconds")]
    pub r#wait_interval_in_seconds: Box<i32>,
}
