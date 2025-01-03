#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryMavenConfig {
    /// The repository with this flag will allow publishing the same
    /// snapshot versions.
    #[builder(into, default)]
    #[serde(rename = "allowSnapshotOverwrites")]
    pub r#allow_snapshot_overwrites: Box<Option<bool>>,
    /// Version policy defines the versions that the registry will accept.
    /// Default value is `VERSION_POLICY_UNSPECIFIED`.
    /// Possible values are: `VERSION_POLICY_UNSPECIFIED`, `RELEASE`, `SNAPSHOT`.
    #[builder(into, default)]
    #[serde(rename = "versionPolicy")]
    pub r#version_policy: Box<Option<String>>,
}
