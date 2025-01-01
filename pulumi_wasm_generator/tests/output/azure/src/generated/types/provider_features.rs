#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderFeatures {
    #[builder(into, default)]
    #[serde(rename = "apiManagement")]
    pub r#api_management: Box<Option<super::types::ProviderFeaturesApiManagement>>,
    #[builder(into, default)]
    #[serde(rename = "appConfiguration")]
    pub r#app_configuration: Box<Option<super::types::ProviderFeaturesAppConfiguration>>,
    #[builder(into, default)]
    #[serde(rename = "applicationInsights")]
    pub r#application_insights: Box<Option<super::types::ProviderFeaturesApplicationInsights>>,
    #[builder(into, default)]
    #[serde(rename = "cognitiveAccount")]
    pub r#cognitive_account: Box<Option<super::types::ProviderFeaturesCognitiveAccount>>,
    #[builder(into, default)]
    #[serde(rename = "keyVault")]
    pub r#key_vault: Box<Option<super::types::ProviderFeaturesKeyVault>>,
    #[builder(into, default)]
    #[serde(rename = "logAnalyticsWorkspace")]
    pub r#log_analytics_workspace: Box<Option<super::types::ProviderFeaturesLogAnalyticsWorkspace>>,
    #[builder(into, default)]
    #[serde(rename = "machineLearning")]
    pub r#machine_learning: Box<Option<super::types::ProviderFeaturesMachineLearning>>,
    #[builder(into, default)]
    #[serde(rename = "managedDisk")]
    pub r#managed_disk: Box<Option<super::types::ProviderFeaturesManagedDisk>>,
    #[builder(into, default)]
    #[serde(rename = "netapp")]
    pub r#netapp: Box<Option<super::types::ProviderFeaturesNetapp>>,
    #[builder(into, default)]
    #[serde(rename = "postgresqlFlexibleServer")]
    pub r#postgresql_flexible_server: Box<Option<super::types::ProviderFeaturesPostgresqlFlexibleServer>>,
    #[builder(into, default)]
    #[serde(rename = "recoveryService")]
    pub r#recovery_service: Box<Option<super::types::ProviderFeaturesRecoveryService>>,
    #[builder(into, default)]
    #[serde(rename = "recoveryServicesVaults")]
    pub r#recovery_services_vaults: Box<Option<super::types::ProviderFeaturesRecoveryServicesVaults>>,
    #[builder(into, default)]
    #[serde(rename = "resourceGroup")]
    pub r#resource_group: Box<Option<super::types::ProviderFeaturesResourceGroup>>,
    #[builder(into, default)]
    #[serde(rename = "storage")]
    pub r#storage: Box<Option<super::types::ProviderFeaturesStorage>>,
    #[builder(into, default)]
    #[serde(rename = "subscription")]
    pub r#subscription: Box<Option<super::types::ProviderFeaturesSubscription>>,
    #[builder(into, default)]
    #[serde(rename = "templateDeployment")]
    pub r#template_deployment: Box<Option<super::types::ProviderFeaturesTemplateDeployment>>,
    #[builder(into, default)]
    #[serde(rename = "virtualMachine")]
    pub r#virtual_machine: Box<Option<super::types::ProviderFeaturesVirtualMachine>>,
    #[builder(into, default)]
    #[serde(rename = "virtualMachineScaleSet")]
    pub r#virtual_machine_scale_set: Box<Option<super::types::ProviderFeaturesVirtualMachineScaleSet>>,
}
