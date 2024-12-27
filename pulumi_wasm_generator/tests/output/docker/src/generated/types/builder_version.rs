#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum BuilderVersion {
    /// The first generation builder for Docker Daemon
    BuilderV1,
    /// The builder based on moby/buildkit project
    BuilderBuildKit,
}
