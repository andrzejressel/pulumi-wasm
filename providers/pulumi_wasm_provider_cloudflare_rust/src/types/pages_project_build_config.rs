#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PagesProjectBuildConfig {
    /// Enable build caching for the project.
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Box<Option<bool>>,
    /// Command used to build project.
    #[serde(rename = "buildCommand")]
    pub r#build_command: Box<Option<String>>,
    /// Output directory of the build.
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Box<Option<String>>,
    /// Your project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty.
    #[serde(rename = "rootDir")]
    pub r#root_dir: Box<Option<String>>,
    /// The classifying tag for analytics.
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Box<Option<String>>,
    /// The auth token for analytics.
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Box<Option<String>>,
}
