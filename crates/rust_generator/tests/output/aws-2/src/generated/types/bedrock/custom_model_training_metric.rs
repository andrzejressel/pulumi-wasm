#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomModelTrainingMetric {
    /// Loss metric associated with the customization job.
    #[builder(into)]
    #[serde(rename = "trainingLoss")]
    pub r#training_loss: Box<f64>,
}
