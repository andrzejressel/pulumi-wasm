#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInferenceProfilesInferenceProfileSummaryModel {
    /// The Amazon Resource Name (ARN) of the model.
    #[builder(into)]
    #[serde(rename = "modelArn")]
    pub r#model_arn: Box<String>,
}
