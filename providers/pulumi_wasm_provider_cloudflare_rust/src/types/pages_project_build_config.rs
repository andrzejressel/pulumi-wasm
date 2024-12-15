#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PagesProjectBuildConfig {
    /// Enable build caching for the project.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Box<Option<bool>>,
    /// Command used to build project.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "buildCommand")]
    pub r#build_command: Box<Option<String>>,
    /// Output directory of the build.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Box<Option<String>>,
    /// Your project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "rootDir")]
    pub r#root_dir: Box<Option<String>>,
    /// The classifying tag for analytics.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Box<Option<String>>,
    /// The auth token for analytics.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Box<Option<String>>,
}
