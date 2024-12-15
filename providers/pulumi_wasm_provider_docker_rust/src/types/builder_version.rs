//! The version of the Docker builder.

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum BuilderVersion {
    /// The first generation builder for Docker Daemon
    #[serde(rename = "BuilderV1")]
    BuilderV1,
    /// The builder based on moby/buildkit project
    #[serde(rename = "BuilderBuildKit")]
    BuilderBuildKit,
}
