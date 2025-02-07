#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamProcessorDataSharingPreference {
    /// Whether you are sharing data with Rekognition to improve model performance.
    #[builder(into)]
    #[serde(rename = "optIn")]
    pub r#opt_in: Box<bool>,
}
