#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageImageTestsConfiguration {
    /// Whether image tests are enabled.
    #[builder(into)]
    #[serde(rename = "imageTestsEnabled")]
    pub r#image_tests_enabled: Box<bool>,
    /// Number of minutes before image tests time out.
    #[builder(into)]
    #[serde(rename = "timeoutMinutes")]
    pub r#timeout_minutes: Box<i32>,
}
