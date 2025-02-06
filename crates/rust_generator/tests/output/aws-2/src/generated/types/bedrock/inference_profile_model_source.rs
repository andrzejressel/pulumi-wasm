#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InferenceProfileModelSource {
    /// The Amazon Resource Name (ARN) of the model.
    #[builder(into)]
    #[serde(rename = "copyFrom")]
    pub r#copy_from: Box<String>,
}
