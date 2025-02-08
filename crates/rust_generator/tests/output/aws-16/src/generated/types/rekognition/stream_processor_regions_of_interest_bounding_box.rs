#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StreamProcessorRegionsOfInterestBoundingBox {
    /// Height of the bounding box as a ratio of the overall image height.
    #[builder(into, default)]
    #[serde(rename = "height")]
    pub r#height: Box<Option<f64>>,
    /// Left coordinate of the bounding box as a ratio of overall image width.
    #[builder(into, default)]
    #[serde(rename = "left")]
    pub r#left: Box<Option<f64>>,
    /// Top coordinate of the bounding box as a ratio of overall image height.
    #[builder(into, default)]
    #[serde(rename = "top")]
    pub r#top: Box<Option<f64>>,
    /// Width of the bounding box as a ratio of the overall image width.
    #[builder(into, default)]
    #[serde(rename = "width")]
    pub r#width: Box<Option<f64>>,
}
