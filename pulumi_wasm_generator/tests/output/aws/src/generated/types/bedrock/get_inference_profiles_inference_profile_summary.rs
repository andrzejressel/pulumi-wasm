#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInferenceProfilesInferenceProfileSummary {
    /// The time at which the inference profile was created.
    #[builder(into)]
    #[serde(rename = "createdAt")]
    pub r#created_at: Box<String>,
    /// The description of the inference profile.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The Amazon Resource Name (ARN) of the inference profile.
    #[builder(into)]
    #[serde(rename = "inferenceProfileArn")]
    pub r#inference_profile_arn: Box<String>,
    /// The unique identifier of the inference profile.
    #[builder(into)]
    #[serde(rename = "inferenceProfileId")]
    pub r#inference_profile_id: Box<String>,
    /// The name of the inference profile.
    #[builder(into)]
    #[serde(rename = "inferenceProfileName")]
    pub r#inference_profile_name: Box<String>,
    /// A list of information about each model in the inference profile. See `models`.
    #[builder(into)]
    #[serde(rename = "models")]
    pub r#models: Box<Vec<super::super::types::bedrock::GetInferenceProfilesInferenceProfileSummaryModel>>,
    /// The status of the inference profile. `ACTIVE` means that the inference profile is available to use.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// The type of the inference profile. `SYSTEM_DEFINED` means that the inference profile is defined by Amazon Bedrock.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The time at which the inference profile was last updated.
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: Box<String>,
}
