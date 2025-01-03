#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamProcessorSettingsConnectedHome {
    /// Specifies what you want to detect in the video, such as people, packages, or pets. The current valid labels you can include in this list are: `PERSON`, `PET`, `PACKAGE`, and `ALL`.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<String>>>,
    /// Minimum confidence required to label an object in the video.
    #[builder(into, default)]
    #[serde(rename = "minConfidence")]
    pub r#min_confidence: Box<Option<f64>>,
}
