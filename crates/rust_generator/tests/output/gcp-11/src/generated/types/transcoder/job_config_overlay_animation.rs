#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobConfigOverlayAnimation {
    /// Display overlay object with fade animation.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "animationFade")]
    pub r#animation_fade: Box<Option<super::super::types::transcoder::JobConfigOverlayAnimationAnimationFade>>,
}
