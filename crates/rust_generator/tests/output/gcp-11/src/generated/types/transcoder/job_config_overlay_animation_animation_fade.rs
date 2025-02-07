#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobConfigOverlayAnimationAnimationFade {
    /// The time to end the fade animation, in seconds.
    #[builder(into, default)]
    #[serde(rename = "endTimeOffset")]
    pub r#end_time_offset: Box<Option<String>>,
    /// Required. Type of fade animation: `FADE_IN` or `FADE_OUT`.
    /// The possible values are:
    /// * `FADE_TYPE_UNSPECIFIED`: The fade type is not specified.
    /// * `FADE_IN`: Fade the overlay object into view.
    /// * `FADE_OUT`: Fade the overlay object out of view.
    /// Possible values are: `FADE_TYPE_UNSPECIFIED`, `FADE_IN`, `FADE_OUT`.
    #[builder(into)]
    #[serde(rename = "fadeType")]
    pub r#fade_type: Box<String>,
    /// The time to start the fade animation, in seconds.
    #[builder(into, default)]
    #[serde(rename = "startTimeOffset")]
    pub r#start_time_offset: Box<Option<String>>,
    /// Normalized coordinates based on output video resolution.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "xy")]
    pub r#xy: Box<Option<super::super::types::transcoder::JobConfigOverlayAnimationAnimationFadeXy>>,
}
