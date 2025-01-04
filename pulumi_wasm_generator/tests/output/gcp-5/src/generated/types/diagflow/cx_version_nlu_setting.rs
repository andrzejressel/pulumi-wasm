#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxVersionNluSetting {
    /// To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a no-match event will be triggered.
    /// The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used.
    #[builder(into, default)]
    #[serde(rename = "classificationThreshold")]
    pub r#classification_threshold: Box<Option<f64>>,
    /// Indicates NLU model training mode.
    /// * MODEL_TRAINING_MODE_AUTOMATIC: NLU model training is automatically triggered when a flow gets modified. User can also manually trigger model training in this mode.
    /// * MODEL_TRAINING_MODE_MANUAL: User needs to manually trigger NLU model training. Best for large flows whose models take long time to train.
    /// Possible values are: `MODEL_TRAINING_MODE_AUTOMATIC`, `MODEL_TRAINING_MODE_MANUAL`.
    #[builder(into, default)]
    #[serde(rename = "modelTrainingMode")]
    pub r#model_training_mode: Box<Option<String>>,
    /// Indicates the type of NLU model.
    /// * MODEL_TYPE_STANDARD: Use standard NLU model.
    /// * MODEL_TYPE_ADVANCED: Use advanced NLU model.
    /// Possible values are: `MODEL_TYPE_STANDARD`, `MODEL_TYPE_ADVANCED`.
    #[builder(into, default)]
    #[serde(rename = "modelType")]
    pub r#model_type: Box<Option<String>>,
}
