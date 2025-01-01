#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct Features {
    #[builder(into, default)]
    #[serde(rename = "apiManagement")]
    pub r#api_management: Box<Option<super::super::types::config::FeaturesApiManagement>>,
    #[builder(into, default)]
    #[serde(rename = "appConfiguration")]
    pub r#app_configuration: Box<Option<super::super::types::config::FeaturesAppConfiguration>>,
    #[builder(into, default)]
    #[serde(rename = "applicationInsights")]
    pub r#application_insights: Box<Option<super::super::types::config::FeaturesApplicationInsights>>,
    #[builder(into, default)]
    #[serde(rename = "cognitiveAccount")]
    pub r#cognitive_account: Box<Option<super::super::types::config::FeaturesCognitiveAccount>>,
    #[builder(into, default)]
    #[serde(rename = "keyVault")]
    pub r#key_vault: Box<Option<super::super::types::config::FeaturesKeyVault>>,
    #[builder(into, default)]
    #[serde(rename = "logAnalyticsWorkspace")]
    pub r#log_analytics_workspace: Box<Option<super::super::types::config::FeaturesLogAnalyticsWorkspace>>,
    #[builder(into, default)]
    #[serde(rename = "machineLearning")]
    pub r#machine_learning: Box<Option<super::super::types::config::FeaturesMachineLearning>>,
    #[builder(into, default)]
    #[serde(rename = "managedDisk")]
    pub r#managed_disk: Box<Option<super::super::types::config::FeaturesManagedDisk>>,
    #[builder(into, default)]
    #[serde(rename = "netapp")]
    pub r#netapp: Box<Option<super::super::types::config::FeaturesNetapp>>,
    #[builder(into, default)]
    #[serde(rename = "postgresqlFlexibleServer")]
    pub r#postgresql_flexible_server: Box<Option<super::super::types::config::FeaturesPostgresqlFlexibleServer>>,
    #[builder(into, default)]
    #[serde(rename = "recoveryService")]
    pub r#recovery_service: Box<Option<super::super::types::config::FeaturesRecoveryService>>,
    #[builder(into, default)]
    #[serde(rename = "recoveryServicesVaults")]
    pub r#recovery_services_vaults: Box<Option<super::super::types::config::FeaturesRecoveryServicesVaults>>,
    #[builder(into, default)]
    #[serde(rename = "resourceGroup")]
    pub r#resource_group: Box<Option<super::super::types::config::FeaturesResourceGroup>>,
    #[builder(into, default)]
    #[serde(rename = "storage")]
    pub r#storage: Box<Option<super::super::types::config::FeaturesStorage>>,
    #[builder(into, default)]
    #[serde(rename = "subscription")]
    pub r#subscription: Box<Option<super::super::types::config::FeaturesSubscription>>,
    #[builder(into, default)]
    #[serde(rename = "templateDeployment")]
    pub r#template_deployment: Box<Option<super::super::types::config::FeaturesTemplateDeployment>>,
    #[builder(into, default)]
    #[serde(rename = "virtualMachine")]
    pub r#virtual_machine: Box<Option<super::super::types::config::FeaturesVirtualMachine>>,
    #[builder(into, default)]
    #[serde(rename = "virtualMachineScaleSet")]
    pub r#virtual_machine_scale_set: Box<Option<super::super::types::config::FeaturesVirtualMachineScaleSet>>,
}
