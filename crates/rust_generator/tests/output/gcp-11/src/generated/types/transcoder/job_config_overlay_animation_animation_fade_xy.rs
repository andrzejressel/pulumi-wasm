#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobConfigOverlayAnimationAnimationFadeXy {
    /// Normalized x coordinate.
    #[builder(into, default)]
    #[serde(rename = "x")]
    pub r#x: Box<Option<f64>>,
    /// Normalized y coordinate.
    #[builder(into, default)]
    #[serde(rename = "y")]
    pub r#y: Box<Option<f64>>,
}
