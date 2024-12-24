#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum BuilderVersion {
    /// The first generation builder for Docker Daemon
    BuilderV1,
    /// The builder based on moby/buildkit project
    BuilderBuildKit,
}
