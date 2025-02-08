#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AiFeatureStoreEntityTypeMonitoringConfig {
    /// Threshold for categorical features of anomaly detection. This is shared by all types of Featurestore Monitoring for categorical features (i.e. Features with type (Feature.ValueType) BOOL or STRING).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "categoricalThresholdConfig")]
    pub r#categorical_threshold_config: Box<Option<super::super::types::vertex::AiFeatureStoreEntityTypeMonitoringConfigCategoricalThresholdConfig>>,
    /// The config for ImportFeatures Analysis Based Feature Monitoring.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "importFeaturesAnalysis")]
    pub r#import_features_analysis: Box<Option<super::super::types::vertex::AiFeatureStoreEntityTypeMonitoringConfigImportFeaturesAnalysis>>,
    /// Threshold for numerical features of anomaly detection. This is shared by all objectives of Featurestore Monitoring for numerical features (i.e. Features with type (Feature.ValueType) DOUBLE or INT64).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "numericalThresholdConfig")]
    pub r#numerical_threshold_config: Box<Option<super::super::types::vertex::AiFeatureStoreEntityTypeMonitoringConfigNumericalThresholdConfig>>,
    /// The config for Snapshot Analysis Based Feature Monitoring.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "snapshotAnalysis")]
    pub r#snapshot_analysis: Box<Option<super::super::types::vertex::AiFeatureStoreEntityTypeMonitoringConfigSnapshotAnalysis>>,
}
