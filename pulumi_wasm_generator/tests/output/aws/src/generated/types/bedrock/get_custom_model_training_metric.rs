#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCustomModelTrainingMetric {
    /// Loss metric associated with the customization job.
    #[builder(into)]
    #[serde(rename = "trainingLoss")]
    pub r#training_loss: Box<f64>,
}