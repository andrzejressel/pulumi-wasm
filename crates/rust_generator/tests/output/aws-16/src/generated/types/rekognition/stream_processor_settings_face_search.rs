#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamProcessorSettingsFaceSearch {
    /// ID of a collection that contains faces that you want to search for.
    #[builder(into)]
    #[serde(rename = "collectionId")]
    pub r#collection_id: Box<String>,
    /// Minimum face match confidence score that must be met to return a result for a recognized face.
    #[builder(into, default)]
    #[serde(rename = "faceMatchThreshold")]
    pub r#face_match_threshold: Box<Option<f64>>,
}
