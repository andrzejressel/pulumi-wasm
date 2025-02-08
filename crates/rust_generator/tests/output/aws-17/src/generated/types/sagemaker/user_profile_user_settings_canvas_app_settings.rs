#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UserProfileUserSettingsCanvasAppSettings {
    /// The model deployment settings for the SageMaker Canvas application. See Direct Deploy Settings below.
    #[builder(into, default)]
    #[serde(rename = "directDeploySettings")]
    pub r#direct_deploy_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsDirectDeploySettings>>,
    /// The settings for running Amazon EMR Serverless jobs in SageMaker Canvas. See `emr_serverless_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "emrServerlessSettings")]
    pub r#emr_serverless_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsEmrServerlessSettings>>,
    #[builder(into, default)]
    #[serde(rename = "generativeAiSettings")]
    pub r#generative_ai_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsGenerativeAiSettings>>,
    /// The settings for connecting to an external data source with OAuth. See Identity Provider OAuth Settings below.
    #[builder(into, default)]
    #[serde(rename = "identityProviderOauthSettings")]
    pub r#identity_provider_oauth_settings: Box<Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsIdentityProviderOauthSetting>>>,
    /// The settings for document querying. See Kendra Settings below.
    #[builder(into, default)]
    #[serde(rename = "kendraSettings")]
    pub r#kendra_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsKendraSettings>>,
    /// The model registry settings for the SageMaker Canvas application. See Model Register Settings below.
    #[builder(into, default)]
    #[serde(rename = "modelRegisterSettings")]
    pub r#model_register_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsModelRegisterSettings>>,
    /// Time series forecast settings for the Canvas app. See Time Series Forecasting Settings below.
    #[builder(into, default)]
    #[serde(rename = "timeSeriesForecastingSettings")]
    pub r#time_series_forecasting_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsTimeSeriesForecastingSettings>>,
    /// The workspace settings for the SageMaker Canvas application. See Workspace Settings below.
    #[builder(into, default)]
    #[serde(rename = "workspaceSettings")]
    pub r#workspace_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsWorkspaceSettings>>,
}
