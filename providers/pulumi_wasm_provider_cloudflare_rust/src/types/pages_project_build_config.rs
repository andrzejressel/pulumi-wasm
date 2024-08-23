#[derive(serde::Serialize)]
pub struct PagesProjectBuildConfig {
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Box<Option<bool>>,
    #[serde(rename = "buildCommand")]
    pub r#build_command: Box<Option<String>>,
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Box<Option<String>>,
    #[serde(rename = "rootDir")]
    pub r#root_dir: Box<Option<String>>,
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Box<Option<String>>,
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Box<Option<String>>,
}
