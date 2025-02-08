#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EndpointDeploymentConfig {
    /// Automatic rollback configuration for handling endpoint deployment failures and recovery. See Auto Rollback Configuration.
    #[builder(into, default)]
    #[serde(rename = "autoRollbackConfiguration")]
    pub r#auto_rollback_configuration: Box<Option<super::super::types::sagemaker::EndpointDeploymentConfigAutoRollbackConfiguration>>,
    /// Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default. See Blue Green Update Config.
    #[builder(into, default)]
    #[serde(rename = "blueGreenUpdatePolicy")]
    pub r#blue_green_update_policy: Box<Option<super::super::types::sagemaker::EndpointDeploymentConfigBlueGreenUpdatePolicy>>,
    /// Specifies a rolling deployment strategy for updating a SageMaker endpoint. See Rolling Update Policy.
    #[builder(into, default)]
    #[serde(rename = "rollingUpdatePolicy")]
    pub r#rolling_update_policy: Box<Option<super::super::types::sagemaker::EndpointDeploymentConfigRollingUpdatePolicy>>,
}
