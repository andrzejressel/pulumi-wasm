#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PagesProjectBuildConfig {
    /// Enable build caching for the project.
    #[builder(into, default)]
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Box<Option<bool>>,
    /// Command used to build project.
    #[builder(into, default)]
    #[serde(rename = "buildCommand")]
    pub r#build_command: Box<Option<String>>,
    /// Output directory of the build.
    #[builder(into, default)]
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Box<Option<String>>,
    /// Your project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty.
    #[builder(into, default)]
    #[serde(rename = "rootDir")]
    pub r#root_dir: Box<Option<String>>,
    /// The classifying tag for analytics.
    #[builder(into, default)]
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Box<Option<String>>,
    /// The auth token for analytics.
    #[builder(into, default)]
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Box<Option<String>>,
}
